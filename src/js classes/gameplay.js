export class AbilityManager{
    abilites = {
        0: new Ability("Water Dance", "heal", 10, 4, "Little heal (10hp)"),
        1: new Ability("Quick Stab", "damage", 25, 1, "Quick damage (25hp)"),

    }

    constructor(){

    }

    
    get_ability(id) {
        return this.abilites[id];
    }

    get_affected_cells(id, turnOwner){
        let ability = this.abilites[id];
        let x = turnOwner[0];
        let y = turnOwner[1];
        let res = [[x-ability.range, y], [x+ability.range, y]];
        return res;
    }
}

class Ability{
    constructor(name, effect, amount, range, description){
        this.name = name;
        this.range = range;
        this.effect = effect;
        this.amount = amount;
        this.description = description;
    }

}