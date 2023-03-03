export default class Letter {
    private _size: number
    private _percents: string
    private _name: string


    constructor(size: number, percents: string, name: string) {
        this._size = size;
        this._name = name;
        this._percents = percents;
    }


    get percents(): string {
        return this._percents;
    }

    set percents(value: string) {
        this._percents = value;
    }

    get size(): number {
        return this._size;
    }

    set size(value: number) {
        this._size = value;
    }

    get name(): string {
        return this._name;
    }

    set name(value: string) {
        this._name = value;
    }
}