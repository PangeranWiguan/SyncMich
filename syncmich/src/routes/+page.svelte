<script lang="ts">
  import { core, dialog } from '@tauri-apps/api';

  let serverUrl = "";
  let apiKey = "";
  let connectionStatus = "Enter your server details to get started.";
  let selectedFolder: string | null = null;
  let folderError: string | null = null;


  // This is the function our button will call.
  async function testConnection() {
    connectionStatus = "Testing connection...";

    try {
        // This calls our Rust function named 'test_immich_connection'
        const result: string = await core.invoke("test_immich_connection", {
            serverUrl, // This passes the value from the input box to Rust
            apiKey,    // This also gets passed to Rust
        });

        // This will run if the Rust function was successful
        connectionStatus = `✅ ${result}`;
    } catch (error) {
        // This will run if the Rust function returned an error
        connectionStatus = `❌ Connection Error: ${error}`;
    }
}
async function selectFolder() {
  folderError = null; // Clear any previous errors
  try {
    // This opens the native OS folder selection dialog
    const result = await dialog.open({
      directory: true, // We want to select a directory
      multiple: false, // We only want to select one
      title: "Select a folder to sync with Immich"
    });

    if (typeof result === 'string') {
      // User selected a folder
      selectedFolder = result;
    } else {
      // User cancelled the dialog, do nothing.
      console.log("User cancelled folder selection.");
    }
  } catch (err) {
    // Handle potential errors from the dialog
    folderError = "Could not open folder dialog: " + err;
    console.error(folderError);
  }
}
</script>

<main class="container">
  <h1>Syncmich Settings</h1>
  <p>Connect to your self-hosted Immich server.</p>

  <div class="form-group">
    <label for="server-url">Server URL</label>
    <input
      id="server-url"
      type="text"
      placeholder="e.g., http://192.168.1.100:2283"
      bind:value={serverUrl}
    />
  </div>

  <div class="form-group">
    <label for="api-key">API Key</label>
    <input
      id="api-key"
      type="password"
      placeholder="Paste your API key here"
      bind:value={apiKey}
    />
  </div>

  <!-- FOLDER SELECTION SECTION -->
<div class="folder-selection">
  <h2>Folder to Sync</h2>
  <p class="description">Select a folder on your computer to automatically back up.</p>
  <button on:click={selectFolder}>Select Folder...</button>

  {#if selectedFolder}
    <div class="selected-path">
      <p>Watching:</p>
      <code>{selectedFolder}</code>
    </div>
  {/if}
</div>

  <button on:click={testConnection}>Test Connection</button>

  <div class="status">
    <p>{connectionStatus}</p>
  </div>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
    --primary-color: #333;
    --secondary-color: #666;
    --background-color: #f4f4f4;
    --input-bg-color: #fff;
    --button-bg-color: #007aff;
    --button-text-color: #fff;
  }

  .container {
    max-width: 500px;
    margin: 40px auto;
    padding: 20px;
    text-align: center;
  }

  .form-group {
    margin-bottom: 20px;
    text-align: left;
  }

  label {
    display: block;
    margin-bottom: 5px;
    color: var(--secondary-color);
  }

  input {
    width: 100%;
    padding: 10px;
    font-size: 16px;
    border: 1px solid #ccc;
    border-radius: 5px;
    box-sizing: border-box; /* Important for padding */
  }

  button {
    width: 100%;
    padding: 12px;
    font-size: 16px;
    font-weight: bold;
    color: var(--button-text-color);
    background-color: var(--button-bg-color);
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  button:hover {
    background-color: #0056b3;
  }

  .status {
    margin-top: 20px;
    color: var(--secondary-color);
  }
  .folder-selection {
  margin-top: 40px;
  padding-top: 20px;
  border-top: 1px solid #eee;
}

.folder-selection h2 {
  margin-bottom: 5px;
}

.folder-selection .description {
  margin-bottom: 20px;
  color: var(--secondary-color);
}

.selected-path {
  margin-top: 20px;
  padding: 10px;
  background-color: #f0f0f0;
  border-radius: 5px;
  text-align: left;
}

.selected-path p {
  margin: 0 0 5px 0;
  font-size: 0.9em;
  color: var(--secondary-color);
}

.selected-path code {
  word-wrap: break-word; /* Make sure long paths don't break the layout */
}
</style>