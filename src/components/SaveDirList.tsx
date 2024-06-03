import React from "react";

export interface PossibleDir {
  directory: string,
  score: number
}

export interface SaveDirListProps {
  searchResult: PossibleDir[];
  selectedDir: string;
  handleRadioChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const SaveDirList: React.FC<SaveDirListProps> = ({ searchResult, selectedDir, handleRadioChange }) => {
  return (
    <div className="path-container scroll">
      {searchResult.length === 0 ? (
        <div className="no-saves-desc">
          <label className="radio-label">
            <input
              type="radio"
              name="radio-checkbox-list"
              value={12}
              disabled
              checked
              style={{ padding: '1px' }}
              onChange={handleRadioChange} 
            />
            Try searching for a game's save directory above.
          </label>
        </div>
      ) : (
        <>
          {searchResult.map((item, index) => (
            <div key={index} className="path-item">
              <label 
                style={index === 0 ? { color: '#55ccff' } : {}} 
                className="radio-label"
              >
                <input
                  type="radio"
                  name="radio-checkbox-list"
                  value={item.directory}
                  checked={selectedDir === item.directory}
                  onChange={handleRadioChange}
                />
                {item.directory}
              </label>
            </div>
          ))}
        </>
      )}
    </div>
  );
};
