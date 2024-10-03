import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function Motor() {

    async function off() {
        await invoke("off");
      }

    return (
        <div>
            <h3>Motor</h3>
            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    off();
                }}
            >
                <button type="submit">Forward</button>
            </form>
            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    off();
                }}
            >
                <button type="submit">Backward</button>
            </form>
            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    off();
                }}
            >
                <button type="submit">Stop</button>
            </form>
        </div>
    );
}

export default Motor;