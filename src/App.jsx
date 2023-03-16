import { invoke } from "@tauri-apps/api";
import { JSXElementConstructor, ReactElement, ReactFragment, ReactPortal, useContext, useEffect, useState } from "react";
import "./App.css";
import {AbilityManager} from './js classes/gameplay.js';

function App() {
  var pieces = [[{name: "Tower", position:[0,0]},
                {name: "Pawn", position:[0,1]},
                {name: "", position:[0,2]},
                {name: "blanka", position:[0,3]},
                {name: "blnak", position:[0,4]},
                {name: "balnk", position:[0,5]},
                {name: "blak", position:[0,6]},
                {name: "bark", position:[0,7]},
                {name: "M", position:[0,8]},
                { name: "Pawn", position:[0,9]},
                { name: "Tower", position:[0,10]},
                { name: "Tower", position:[0,11]},
              ], 
              [{name: "Tower", position:[1,0]},
              {name: "Pawn", position:[1,1]},
              {name: "blenk", position:[1,2]},
              {name: "", position:[1,3]},
              {name: "blnak", position:[1,4]},
              {name: "balnk", position:[1,5]},
              {name: "blak", position:[1,6]},
              {name: "bark", position:[1,7]},
              {name: "A", position:[1,8]},
              { name: "Pawn", position:[1,9]},
              { name: "Tower", position:[1,10]},
              { name: "Tower", position:[1,11]},
            ], 
            [{name: "Tower", position:[2,0]},
                {name: "Pawn", position:[2,1]},
                {name: "blenk", position:[2,2]},
                {name: "blanka", position:[2,3]},
                {name: "", position:[2,4]},
                {name: "balnk", position:[2,5]},
                {name: "blak", position:[2,6]},
                {name: "bark", position:[2,7]},
                {name: "S", position:[2,8]},
                { name: "Pawn", position:[2,9]},
                { name: "Tower", position:[2,10]},
                { name: "Tower", position:[2,11]},
              ], 
              [{name: "Tower", position:[3,0]},
                {name: "Pawn", position:[3,1]},
                {name: "blenk", position:[3,2]},
                {name: "blanka", position:[3,3]},
                {name: "blnak", position:[3,4]},
                {name: "", position:[3,5]},
                {name: "blak", position:[3,6]},
                {name: "bark", position:[3,7]},
                {name: "T", position:[3,8]},
                { name: "Pawn", position:[3,9]},
                { name: "Tower", position:[3,10]},
                { name: "Tower", position:[3,11]},
              ], 
              [{name: "Tower", position:[4,0]},
                {name: "Pawn", position:[4,1]},
                {name: "blenk", position:[4,2]},
                {name: "blanka", position:[4,3]},
                {name: "blnak", position:[4,4]},
                {name: "balnk", position:[4,5]},
                {name: "", position:[4,6]},
                {name: "bark", position:[4,7]},
                {name: "E", position:[4,8]},
                {name: "Pawn", position:[4,9]},
                {name: "Tower", position:[4,10]},
                {name: "Tower", position:[4,11]},
              ], 
              [{name: "Tower", position:[5,0]},
                {name: "Pawn", position:[5,1]},
                {name: "blenk", position:[5,2]},
                {name: "blanka", position:[5,3]},
                {name: "blnak", position:[5,4]},
                {name: "balnk", position:[5,5]},
                {name: "blak", position:[5,6]},
                {name: "", position:[5,7]},
                {name: "R", position:[5,8]},
                {name: "Pawn", position:[5,9]},
                {name: "Tower", position:[5,10]},
                {name: "Tower", position:[5,11]},
              ]
            ];
  
  const [turn, setTurn] = useState(0);
  const [board, setBoard] = useState(pieces);
  const [turnOwner, setTurnOwner] = useState([10,10]);
  const [ability, setAbility] = useState(null);

  function passTurn() {
    setTurn(turn + 1);
    invoke("pass_turn");

    Promise.resolve(invoke("get_board"))
    .then((value) => {setBoard(value);});
    Promise.resolve(invoke("get_turn_owner"))
    .then((value) => {setTurnOwner(value);});

    console.log(turnOwner);
  }

  return (
    <div className="container">
      <button onClick={() => passTurn()} className="turn-button">{turn}</button>
      
      <Grid turnOwner={turnOwner} board={board} ability={ability}/>
      <ActionBar setAbility={setAbility}/>
    </div>
  );
}
export default App;


function ActionBar(props){
  const ability_manager = new AbilityManager();

  function abilitysetting(id){
    let ability = ability_manager.get_ability(id);
    props.setAbility(ability);
  }

  return (
    <div className="action-bar">
      <button onClick={() => abilitysetting(0)}>habilidad 1</button>
      <button onClick={() => abilitysetting(1)}>habilidad 2</button>
      <button onClick={() => props.setAbility(null)}>Cancel ability</button>
    </div>
  )
}

function Grid(props){

  function isOnRange(piece_position){
    if(props.ability != null){
      let x_diff =  Math.abs(piece_position[0] - props.turnOwner[0])
      let y_diff = Math.abs(piece_position[1] - props.turnOwner[1])
      let is_on_range = (x_diff + y_diff <= props.ability.range)

      return is_on_range;
    } else {
      return false;
    }
  }

  return (
    <div className="grid">
        {props.board.map( row => (
        row.map( piece =>{
        if(props.turnOwner[0] == piece.position[0] & props.turnOwner[1] == piece.position[1]){
          return (<PlayingCell name={piece.name}/>)
          
        }
        else if(isOnRange(piece.position)){
          return (<ReachableCell name={piece.name} ability={props.ability}/>)
        }
        else{
          return (<Cell name={piece.name} ability={props.ability}/>)
        }
        })))}
    </div>
  )    
};

function Cell(props){
  function reactToClick(){
    if (props.ability != null){
      alert("Unreachable cell");
    } else {
      console.log("rea")
    }
  }
  return(
    <div className="cell" onClick={() => reactToClick()}>
      <>
        {props.name}
      </>
    </div>
  )
};

function PlayingCell(props){
  return(
    <div className="playing-cell">
      <>
        {props.name}
      </>
    </div>
  )

}

function ReachableCell(props){
  return(
    <div className="affected-cell">
      <>
        {props.name}
      </>
    </div>
  )
};