export interface Mood { epoch_millis: number, value: number }
export interface Text { epoch_millis: number, value: string }
export interface Log { mood_readings: Mood[], meds: Text[], notes: Text[], sleep_entries: Text[] }