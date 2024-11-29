import {invoke} from "@tauri-apps/api/core";
import "../App.css";


export function FileAddForm() {
    function add_exe() {
        // @ts-ignore
        const name = String(document.getElementsByName("exe_name")[0].value);
        // @ts-ignore
        const path = String(document.getElementsByName("exe_path")[0].value);
        if (name === "" || path === "") {
            return;
        }

        invoke("add_executable", {name, path}).then(() => {
            console.log("Added executable " + name + " at " + path);
            // close the form
            // @ts-ignore
            document.getElementsByTagName("form")[0].submit();
        });


    }

    return (
        <form>
            <label>
                Name: <input type="text" name="exe_name"/>
            </label>
            <br/>
            <label>
                Path: <input type="text" name="exe_path"/>
            </label>
            <button type="button" onClick={add_exe}>Add</button>
        </form>
    );
}