import init, {
    installOpfsSahpool,
    installRelaxedIdb,
    switchVfs
} from './dure.js';

// Initialize WASM module
await init();
await installOpfsSahpool();
await installRelaxedIdb();

async function handleMessage(event) {
    const { cmd, ...payload } = event.data;

    try {
        switch (cmd) {
            case 'switch_vfs':
                switchVfs(payload.id);
                break;

            default:
                console.warn(`Unknown command: ${cmd}`);
                postResponse('error', { error: `Unknown command: ${cmd}` });
        }
    } catch (error) {
        console.error(`Error processing ${cmd}:`, error);
        postResponse('error', {
            error: error.message,
            originalCmd: cmd
        });
    }
}

/**
 * Posts a response back to the main thread
 * @param {string} cmd - The command name
 * @param {object} [data] - Additional response data
 */
function postResponse(cmd, data = {}) {
    self.postMessage({
        cmd,
        ...data
    });
}

// Set up message listener
self.addEventListener('message', handleMessage);

// Notify main thread that worker is ready
postResponse('ready');
