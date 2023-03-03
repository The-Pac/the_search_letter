import Letter from "./Letter";
import File from "./File";

export default class Result {
    chars_result: Array<Letter>
    files_result: Array<File>
    duration: number


    constructor(chars_result: Array<Letter>, files_result: Array<File>, duration: number) {
        this.chars_result = chars_result;
        this.files_result = files_result;
        this.duration = duration;
    }


}