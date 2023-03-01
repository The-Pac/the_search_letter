import Letter from "./Letter";

export default class Result {
    chars_result: Array<Letter>
    duration: number


    constructor(chars_result: Array<Letter>, duration: number) {
        this.chars_result = chars_result;
        this.duration = duration;
    }


}