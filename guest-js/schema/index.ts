import Time from "../types/time.type";
export type SchemaTypes = 'string' | 'number' | 'boolean' | 'object' | 'array' | 'date' | 'time';

export interface SchemaItem {
    type: SchemaTypes;
    default?: Function;
    required?: boolean;
    unique?: boolean;
    ref?: string;
}

export interface Schema {
    [key: string]: SchemaItem;
}

export type TypeMap = {
    string: string;
    number: number;
    boolean: boolean;
    date: Date;
    time: Time;
    object: object;
    array: any[];
    buffer: Uint8Array;
    mixed: any;
};

export type InferSchemaType<T extends Schema> = {
    [K in keyof T]?: T[K]['required'] extends true
    ? TypeMap[T[K]['type']]
    : TypeMap[T[K]['type']] | undefined;
} & {
    [K in keyof T as T[K]['required'] extends true ? K : never]-?: TypeMap[T[K]['type']];
};