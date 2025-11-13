import { app, BrowserWindow, Menu } from 'electron';
import path from 'path';
// import isDev from 'electron-is-dev';
// Force check for production
const isDev = process.env.NODE_ENV !== 'production' && !app.isPackaged;
import http from 'http';
import { registerServices } from './ipc-loader';

// Disable security warnings in dev
process.env['ELECTRON_DISABLE_SECURITY_WARNINGS'] = 'true';

let mainWindow: BrowserWindow | null = null;
let creatingWindow = false;

// Single instance lock
const gotSingleInstanceLock = app.requestSingleInstanceLock();

if (!gotSingleInstanceLock) {
  app.quit();
  process.exit(0);
}

app.on('second-instance', () => {
  if (mainWindow) {
    if (mainWindow.isMinimized()) mainWindow.restore();
    mainWindow.focus();
  }
});

// Find available port
async function findAvailablePort(startPort: number, maxAttempts = 10): Promise<number> {
  for (let i = 0; i < maxAttempts; i++) {
    const port = startPort + i;
    try {
      await new Promise<void>((resolve, reject) => {
        const testServer = http.createServer();
        testServer.once('error', reject);
        testServer.once('listening', () => {
          testServer.close();
          resolve();
        });
        testServer.listen(port);
      });
      return port;
    } catch (error) {
      continue;
    }
  }
  throw new Error(`No available ports found between ${startPort} and ${startPort + maxAttempts}`);
}

async function waitForNextServer(startPort: number, maxWaitTime = 60000): Promise<number> {
  const startTime = Date.now();

  return new Promise<number>((resolve, reject) => {
    let currentPort = startPort;
    let portAttempts = 0;
    const maxPortAttempts = 10;

    const checkServer = () => {
      if (Date.now() - startTime > maxWaitTime) {
        reject(new Error('Timeout waiting for Next.js server'));
        return;
      }

      http
        .get(`http://localhost:${currentPort}`, (res) => {
          if (res.statusCode === 200) {
            console.log(`✅ Found Next.js on port ${currentPort}`);
            resolve(currentPort);
          } else {
            setTimeout(checkServer, 1000);
          }
        })
        .on('error', () => {
          // Try next port if this one(in dev mode lol)
          portAttempts++;
          if (portAttempts < maxPortAttempts) {
            currentPort++;
            console.log(`🔄 Trying port ${currentPort}...`);
            setTimeout(checkServer, 500);
          } else {
            // Reset and try from start again
            portAttempts = 0;
            currentPort = startPort;
            setTimeout(checkServer, 1000);
          }
        });
    };

    checkServer();
  });
}

// main window
async function createMainWindow() {
  if (creatingWindow || mainWindow) return;
  creatingWindow = true;

  try {
    mainWindow = new BrowserWindow({
      width: 1200,
      height: 800,
      autoHideMenuBar: true,
      webPreferences: {
        preload: path.join(__dirname, 'preload.js'),
        nodeIntegration: false,
        contextIsolation: true,
      },
    });

    Menu.setApplicationMenu(null);

    if (isDev) {
      const startPort = Number(process.env.PORT ?? 3000);
      console.log('DEV: AAAAAAA waiting for Next.js dev server...');

      try {
        const actualPort = await waitForNextServer(startPort, 60000);
        await mainWindow.loadURL(`http://localhost:${actualPort}`);
        mainWindow.webContents.openDevTools();
      } catch (error) {
        console.error('❌ Failed to connect to Next.js:', error);
        mainWindow.loadURL(`data:text/html,<h1>cunt start Next.js dev server</h1><p>${error}</p>`);
      }
    } else {
      // Production ()
      const indexPath = app.isPackaged
        ? path.join(process.resourcesPath, 'app.asar', 'out', 'index.html')
        : path.join(__dirname, '../out/index.html');

      console.log('loaded:', indexPath);

      // loadFile with proper protocol handling
      await mainWindow.loadFile(indexPath);

      //DevTools for production ( me need it sometimes)
      // mainWindow.webContents.openDevTools();
    }

    mainWindow.on('closed', () => {
      mainWindow = null;
    });
  } catch (error) {
    console.error('win Creation failed:', error);
    mainWindow = null;
  } finally {
    creatingWindow = false;
  }
}

// life
app.on('ready', async () => {
  try {
    console.log('🚀 Starting application...');

    // Auto-register all services as IPC handlers
    const servicesPath = path.join(__dirname, 'services');
    console.log(`services in: ${servicesPath}`);

    await registerServices(servicesPath);
    // create window
    await createMainWindow();
  } catch (error) {
    console.error('❌ Failed to initialize app:', error);
    app.quit();
  }
});
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
app.on('activate', () => {
  if (!mainWindow && !creatingWindow) {
    createMainWindow();
  }
});

// exceptions and crashes
process.on('uncaughtException', (error) => {
  console.error('Uncaught exception:', error);
});

process.on('unhandledRejection', (error) => {
  console.error('Unhandled rejection:', error);
});