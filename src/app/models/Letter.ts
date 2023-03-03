export default class Letter {
    private _number: number
    private _percents: string
    private _char: string


    constructor(number: number,percents: string, char: string) {
        this._number = number;
        this._char = char;
        this._percents = percents;
    }


    get percents(): string {
        return this._percents;
    }

    set percents(value: string) {
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