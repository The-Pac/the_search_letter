export default class Letter {
    private _number: number
    private _percents: number
    private _char: string


    constructor(number: number,percents: number, char: string) {
        this._number = number;
        this._char = char;
        this._percents = percents;
    }


    get percents(): number {
        return this._percents;
    }

    set percents(value: number) {
        this._percents = value;
    }

    get number(): number {
        return this._number;
    }

    set number(value: number) {
        this._number = value;
    }

    get char(): string {
        return this._char;
    }

    set char(value: string) {
        this._char = value;
    }
}