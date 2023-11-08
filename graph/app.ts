// an app that can print mood graphs over a very long period

import { Log } from "./model.ts";


// read in text from file named "sample.json"


const sample: Log = JSON.parse( Deno.readTextFileSync("sample.json"));

// print out the mood readings
console.log(sample.mood_readings);

// group mood readings by day
// create a map from day (string) to list of mood readings (number[])
const byDay: Map<string, number[]> = new Map<string, number[]>();

// for each mood reading
for (const mood of sample.mood_readings) {
    // convert epoch_millis to a date
    const d = new Date(mood.epoch_millis);
    // get the day (yyyy-mm-dd)
    const day = d.toISOString().slice(0, 10);
    // add this mood reading to the list of mood readings for this day
    // if this is the first mood reading for this day, create a new list
    const list = byDay.get(day) ?? [];
    list.push(mood.value);

    // deduplicate entries in the list
    const deduped = [...new Set(list)];
    // sort the list
    deduped.sort((a, b) => a - b);

    // store the list in the map
    byDay.set(day, deduped);
}

// print out the map
console.log(byDay);