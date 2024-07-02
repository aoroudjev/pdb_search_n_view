import React, { useState } from 'react';
import './App.css';
import SearchBar from "./Components/SearchBar";
import ReturnTile from "./Components/ReturnTile"; // Ensure correct import

interface Result {
  title: string,
  description: string,
}

function App() {
  const [results, setResults] = useState<Result[]>([])

  const handleSearch = async (query: string) => {
    try {
      const response = await fetch(`http://localhost:2000/?search_term=${query}`);
      const data = await response.json();
      console.log("Received data from API:", data);

      const transformedResults = data.results.map((result: any) => {
        const title = result.proteinDescription?.recommendedName?.fullName?.value || "No Title";
        const description = `Organism: ${result.organism?.scientificName} (${result.organism?.commonName})`;
        return { title, description };
      });

      console.log("Transformed Results:", transformedResults);
      setResults(transformedResults);
    } catch (error) {
      console.error("Error fetching data: ", error);
    }
  };

  return (
      <div className="App">
        <header className="App-header">
          <SearchBar onSearch={handleSearch} />
          <div className="results">
            {results.map((result, index) => (
                <ReturnTile key={index} result={result} />
            ))}
          </div>
        </header>
      </div>
  );
}

export default App;