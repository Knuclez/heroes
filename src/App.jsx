import { invoke } from "@tauri-apps/api";
import { JSXElementConstructor, ReactElement, ReactFragment, ReactPortal, useContext, useEffect, useState } from "react";
import "./App.css";

function App() {
  return (
    <div className="container">
      {Grid()}
    </div>
  );
}
export default App;


function Grid(){
  var pieces = {"pieces":[{"line":[{name: "Tower"},
                {name: "Pawn"},
                {name: ""},
                {name: "blanka"},
                {name: "blnak"},
                {name: "balnk"},
                {name: "blak"},
                {name: "bark"},
                {name: "M"},
                { name: "Pawn"},
                { name: "Tower"},
                { name: "Tower"},
              ]}, 
              {"line": [{name: "Tower"},
              {name: "Pawn"},
              {name: "blenk"},
              {name: ""},
              {name: "blnak"},
              {name: "balnk"},
              {name: "blak"},
              {name: "bark"},
              {name: "A"},
              { name: "Pawn"},
              { name: "Tower"},
              { name: "Tower"},
            ]}, 
            {"line": [{name: "Tower"},
                {name: "Pawn"},
                {name: "blenk"},
                {name: "blanka"},
                {name: ""},
                {name: "balnk"},
                {name: "blak"},
                {name: "bark"},
                {name: "S"},
                { name: "Pawn"},
                { name: "Tower"},
                { name: "Tower"},
              ]}, 
              {"line":[{name: "Tower"},
                {name: "Pawn"},
                {name: "blenk"},
                {name: "blanka"},
                {name: "blnak"},
                {name: ""},
                {name: "blak"},
                {name: "bark"},
                {name: "T"},
                { name: "Pawn"},
                { name: "Tower"},
                { name: "Tower"},
              ]}, 
              {"line":[{name: "Tower"},
                {name: "Pawn"},
                {name: "blenk"},
                {name: "blanka"},
                {name: "blnak"},
                {name: "balnk"},
                {name: ""},
                {name: "bark"},
                {name: "E"},
                {name: "Pawn"},
                {name: "Tower"},
                {name: "Tower"},
              ]}, 
              {"line":[{name: "Tower"},
                {name: "Pawn"},
                {name: "blenk"},
                {name: "blanka"},
                {name: "blnak"},
                {name: "balnk"},
                {name: "blak"},
                {name: ""},
                {name: "R"},
                {name: "Pawn"},
                {name: "Tower"},
                {name: "Tower"},
              ]}
            ]};
  const [board, setBoard] = useState(pieces);
  const [turnOwner, setTurnOwner] = useState("");

  useEffect(() => {
    Promise.resolve(invoke("start_game"))
    .then((value) => setBoard(value));

  }, [])
  return (
    <div className="grid">
        {board.pieces.map( row => ( row.line.map( piece =>(
          <Cell name={piece.name} turnOwner={turnOwner}/>  
        ))))
        }
    </div>
  )    
};


function Cell(props){
  return(
    <div className="cell">
      <>
        {props.name}
      </>
    </div>
  )
};


