export class AbilityManager{
    abilites = {
        0: new Ability("Water Dance", "heal", 10, 5, 1, 1, "Little heal (10hp)"),
        1: new Ability("Quick Stab", "damage", 25, 1, 1, 1, "Quick damage (25hp)"),

    }

    constructor(){

    }

    
    get_ability(id) {
        return this.abilites[id];
    }
}

class Ability{
    constructor(name, effect, amount, range, vertical_area, horizontal_area, description){
        this.name = name;
        this.range = range;
        this.effect = effect;
        this.amount = amount;
        this.vertical_area = vertical_area;
        this.horizontal_area = horizontal_area;
        this.description = description;
    }

}