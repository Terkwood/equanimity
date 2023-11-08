
function deepestBlue(moods: number[]) : number {
    const smallest = moods.reduce((a, b) => Math.min(a, b));
    return smallest < 1 ? smallest : 0;
}

function brightestRed(moods: number[]) : number {
    const largest = moods.reduce((a, b) => Math.max(a, b));
    return largest > -1 ? largest : 0;
}

function hadEquanimity(moods: number[]) : boolean {
    return moods.find(mood => mood === 0) !== undefined;
}

export function circles(moods: number[]) {
    const red = brightestRed(moods);
    const blue = deepestBlue(moods);
    const equanimity = hadEquanimity(moods);

    const MANIC_CIRCLE = '🔴';
    const DEPRESSED_CIRCLE = '🔵';
    const EQUANIMITY_CIRCLE = '⚪';
    const EMPTY_CIRCLE = '⚫';

    const redCircles = `${MANIC_CIRCLE.repeat(red)}${EMPTY_CIRCLE.repeat(3 - red)}` ;
    const blueCircles = `${EMPTY_CIRCLE.repeat(3 - Math.abs(blue))}${DEPRESSED_CIRCLE.repeat(Math.abs(blue))}`;

    // define a string which shows EQUANIMITY_CIRCLE if equanimity is true, otherwise EMPTY_CIRCLE
    const equanimityCircle = equanimity ? EQUANIMITY_CIRCLE : EMPTY_CIRCLE;

    return `${blueCircles}${equanimityCircle}${redCircles}`;

}