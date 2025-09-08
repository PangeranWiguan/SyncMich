<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let serverUrl = "";
  let apiKey = "";
  let connectionStatus = "Enter your server details to get started.";

  // This is the function our button will call.
  async function testConnection() {
    connectionStatus = "Testing connection...";

    try {
        // This calls our Rust function named 'test_immich_connection'
        const result: string = await invoke("test_immich_connection", {
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
</style>