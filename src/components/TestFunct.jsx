import { Button, DatePicker } from "antd";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";


export default function TestFunct(){
  const [rust_msg, setRustMsg] = useState("");
  const [filePath, setFilePath] = useState("");

  const msg_rust = async () => {
    const response = await invoke('greet', { userName: 'Igor' }).then((response) => response);
    setRustMsg(response.message); // Получение значения параметра message
  }
  msg_rust()

  const openFile = async() => {
    try {
      const selectedPath = await open({multiple: false});
      setFilePath(selectedPath);
    } catch (err){
      console.error(err)
    }
  }

  return (
    <>
      <p className="text-3xl underline">Hello world! Max</p>
      <DatePicker />
      <Button onClick={openFile} title="Open file dialog"/>
    </>
  );

}