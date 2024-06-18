import React, {useState} from 'react';
import './App.css';
import SearchBar from "./Components/SearchBar";
import ResultTile from "./Components/ResultTile";

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
      setResults(data.results); // Assuming the backend returns an object with a "results" array
      console.log(data);
    } catch (error) {
      console.error("Error fetching data: ", error);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <SearchBar onSearch={handleSearch}></SearchBar>
        <div className="results">
          {results.map((result, index) => (
              <ResultTile key={index} result={result}/>
          ))}
        </div>
      </header>
    </div>
  );
}

export default App;
