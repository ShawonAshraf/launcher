import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";

interface Executable {
    id?: number;
    name: string;
    path: string;
}

function App() {
    const [executables, setExecutables] = useState([]);

    useEffect(() => {
        // @ts-ignore
        invoke("get_executables").then((executables: Executable[]) => {
            // @ts-ignore
            setExecutables(executables);
        });
    }, []);

    return (
        <main className="container">
            <h1>Launcher</h1>
            <table>
                <th>Name</th>
                <th>Options</th>

                {
                    executables.map((executable: Executable) => (
                        <tr key={executable.id}>
                            <td>{executable.name}</td>
                            <td>
                                <button onClick={() => {
                                    // @ts-ignore
                                    invoke("run_executable", {path: executable.path});
                                }}>Launch
                                </button>
                            </td>
                        </tr>
                    ))
                }
            </table>
        </main>
    );
}

export default App;
