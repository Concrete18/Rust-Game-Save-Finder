import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from '@tauri-apps/api/clipboard';
import { appWindow } from '@tauri-apps/api/window'
import "./App.css";

import {SavePathList, PossiblePath} from "./components/SavePathList";  // Import the new component

function App() {
  const [gameName, setGameName] = useState("");
  const [searchResult, setSearchResult] = useState<PossiblePath[]>([]);
  const [selectedPath, setSelectedPath] = useState('');

  const handleMinimize = async () => {
    appWindow.minimize()
  };

  const handleClose = async () => {
    appWindow.close()
  };

  const handleRadioChange = (e: any) => {
    const value = e.target.value;
    console.log(value)
    setSelectedPath(value);
  };

  const openFilePath = async () => {
    await invoke("open_path", { path: selectedPath })
  };

  const copyToClipboard = async () => {
    await writeText(selectedPath)
  };

  async function findSave() {
    if (gameName) {
      let saveLocation: PossiblePath[] = await invoke("find_game_save_paths", { gameName })
      setSearchResult(saveLocation);
    } else {
      setSearchResult([]);
    }
  }

  useEffect(() => {
    if (searchResult.length > 0) {
      setSelectedPath(searchResult[0].path);
    }
  }, [searchResult]);

  return (
    <div className="container">

      <div data-tauri-drag-region className="titlebar">
        <div className="titlebar-button" id="titlebar-minimize" onClick={handleMinimize}>
          <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
          />
        </div>
        <div className="titlebar-button" id="titlebar-close" onClick={handleClose}>
          <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </div>
      </div>

      <h1>Game Save Finder</h1>

      <p>To find the possible save locations, type in the Game's name.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          findSave();
        }}
      >
        <input
          id="game-input"
          onChange={(e) => setGameName(e.currentTarget.value)}
          placeholder="Enter Game Name"
        />
        <button type="submit" >Search</button>
      </form>
        <div className="save-display">
          <div className="path-desc">
            The highlighted entry is most likely the save path.
          </div>
          <SavePathList
            searchResult={searchResult}
            selectedPath={selectedPath}
            handleRadioChange={handleRadioChange}
          />
          <div className="button-container">
            <button className={`bottom-button ${searchResult.length == 0 ? 'disabled' : ''}`} onClick={openFilePath} disabled={searchResult.length == 0} >Open in Explorer</button>
            <button className={`bottom-button ${searchResult.length == 0 ? 'disabled' : ''}`} onClick={copyToClipboard} disabled={searchResult.length == 0} >Copy to Clipboard</button>
          </div>
        </div>
    </div>
  );
}

export default App;
