import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from '@tauri-apps/api/clipboard';
import "./App.css";

interface PossiblePath {
  path: string,
  score: number
}

function App() {
  const [gameName, setGameName] = useState("");
  const [searchResult, setSearchResult] = useState<PossiblePath[]>([]);
  const [selectedPath, setSelectedPath] = useState('');

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

  return (
    <div className="container">
      <h1>Game Save Finder</h1>

      <p>Type in the Game name that you want to find the save location for.</p>

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
        <button type="submit">Save Search</button>
      </form>

      {searchResult.length === 0 ? (
        <div>
          <h3>No Save Paths to Show.</h3>
          <p>Try searching for a game's save path.</p>
        </div>
      ) : (
        <div>
          <div className="scroll">
            {searchResult.map((item, index) => (
              <div key={index}>
                <label 
                  style={index === 0 ? { fontWeight: 'bold', color: '#55ccff', padding: '5px' } : {}}>
                  <input
                    type="radio"
                    name="radio-checkbox-list"
                    value={item.path}
                    style = {{padding: '1px'}}
                    checked={selectedPath === item.path}
                    defaultChecked={index === 0}
                    onChange={handleRadioChange}
                  />
                  {item.path}
                </label>
              </div>
            ))}
          </div>
          <div className="button-container">
            <button className="bottom-button" onClick={openFilePath} >Open Possible Save Path</button>
            <button className="bottom-button" onClick={copyToClipboard} >Copy to Clipboard</button>
          </div>
        </div>
      )}

    </div>
  );
}

export default App;
