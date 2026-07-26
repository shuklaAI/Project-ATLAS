import "./App.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [deviceId, setDeviceId] = useState("Loading...");

  useEffect(() => {
    invoke<string>("get_device_id")
      .then((id) => setDeviceId(id))
      .catch((err) => {
        console.error(err);
        setDeviceId("Error");
      });
  }, []);

  return (
    <div className="app">
      <h1>ATLAS DESKTOP</h1>

      <div className="card">
        <h2>Device</h2>

        <p>
          <strong>Name:</strong> Ghost-PC
        </p>

        <p>
          <strong>Device ID:</strong> {deviceId}
        </p>

        <p>
          <strong>Status:</strong> Ready
        </p>

        <p>
          <strong>Network:</strong> Waiting for nearby Atlas devices...
        </p>
      </div>

      <footer>Project Atlas v0.1</footer>
    </div>
  );
}

export default App;