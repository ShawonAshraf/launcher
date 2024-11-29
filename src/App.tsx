import "./App.css";
// @ts-ignore
import { Executable } from "./Types.ts";
import { Table } from "./components/Table";

function App() {
    return (
        <main className="container">
            <h1>Launcher</h1>
            <button id={"addbtn"} onClick={() => {
                alert("Derp!");
            }}
            >Add</button>
            <Table/>
        </main>
    );
}

export default App;
