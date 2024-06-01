import React from "react";

export interface PossiblePath {
  path: string,
  score: number
}

export interface SavePathListProps {
  searchResult: PossiblePath[];
  selectedPath: string;
  handleRadioChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export const SavePathList: React.FC<SavePathListProps> = ({ searchResult, selectedPath, handleRadioChange }) => {
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
            Try searching for a game's save path above.
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
                  value={item.path}
                  style={{ padding: '1px' }}
                  checked={selectedPath === item.path}
                  onChange={handleRadioChange}
                />
                {item.path}
              </label>
            </div>
          ))}
        </>
      )}
    </div>
  );
};
