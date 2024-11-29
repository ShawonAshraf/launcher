import "./App.css";
// @ts-ignore
import {Executable} from "./Types.ts";
import {Table} from "./components/Table";
import {FileAddForm} from "./components/FileAddForm";
import {Modal} from './components/Modal';
import {useState} from 'react';


function App() {
    const [showModal, setShowModal] = useState(false);
    return (
        <main className="container">
            <h1>Launcher</h1>
            <button id="addbtn" onClick={() => setShowModal(true)}>Add</button>
            <Table/>
            <Modal show={showModal} onClose={() => setShowModal(false)}>
                <FileAddForm/>
            </Modal>
        </main>
    );
}

export default App;
