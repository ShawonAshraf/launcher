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
    const [statusId, setStatusId] = useState(-1);

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
                <th>ID</th>
                <th>Name</th>
                <th>Options</th>
                <th>Status ID</th>
                {
                    executables.map((executable: Executable) => (
                        <tr key={executable.id}>
                            <td>{executable.id}</td>
                            <td>{executable.name}</td>
                            <td>
                                <button onClick={() => {
                                    // @ts-ignore
                                    invoke("run_executable", {path: executable.path}).then((statusId: number) => {
                                        setStatusId(statusId);
                                    });
                                }}>Launch
                                </button>

                                <button onClick={() => {
                                    invoke("delete_executable", {id: executable.id}).then(() => {
                                        // @ts-ignore
                                        setExecutables(executables.filter((e: Executable) => e.id !== executable.id));
                                    });
                                }}
                                >Delete</button>
                            </td>
                            <td>{statusId}</td>
                        </tr>
                    ))
                }
            </table>
        </main>
    );
}

export default App;
