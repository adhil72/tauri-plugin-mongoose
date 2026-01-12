import { invoke } from '@tauri-apps/api/core';
import { InferSchemaType, Schema } from "../schema";
import { Time, isTime } from '../types';

export default class Model {
    name: string
    schema: Schema

    constructor(name: string, schema: Schema) {
        this.name = name;
        this.schema = schema;
    }

    async getById(id: string): Promise<InferSchemaType<typeof this.schema>> {
        const doc = await invoke('plugin:mongoose|get_by_id', {
            collection: this.name,
            id
        }) as InferSchemaType<typeof this.schema>;
        return this.validate(doc);
    }

    async validate(doc: InferSchemaType<typeof this.schema>) {
        for (const key in this.schema) {
            const schemaItem = this.schema[key];
            const value = doc[key];

            if (schemaItem.required && (value === undefined || value === null || value === '')) {
                throw new Error(`Property ${key} is required`);
            }

            if (value !== undefined && value !== null) {
                if (schemaItem.type === 'string' && typeof value !== 'string') {
                    throw new Error(`Property ${key} must be a string`);
                }
                if (schemaItem.type === 'number' && typeof value !== 'number') {
                    throw new Error(`Property ${key} must be a number`);
                }
                if (schemaItem.type === 'boolean' && typeof value !== 'boolean') {
                    throw new Error(`Property ${key} must be a boolean`);
                }
                if (schemaItem.type === 'object' && typeof value !== 'object') {
                    throw new Error(`Property ${key} must be an object`);
                }
                if (schemaItem.type === 'array' && !Array.isArray(value)) {
                    throw new Error(`Property ${key} must be an array`);
                }
                if (schemaItem.type === 'date' && !(value instanceof Date)) {
                    throw new Error(`Property ${key} must be a date`);
                }
                if (schemaItem.type === 'time' && !(value instanceof Time)) {
                    throw new Error(`Property ${key} must be a time`);
                }
                if (schemaItem.type === 'time' && !(isTime(value))) {
                    throw new Error(`Property ${key} must be a time`);
                }
            }

            if (schemaItem.default && value === undefined) {
                doc[key] = schemaItem.default();
            }

            if (schemaItem.unique && value !== undefined && value !== null) {
                const existing = await invoke('plugin:mongoose|get_by_id', {
                    collection: this.name,
                    id: value
                });
                if (existing) {
                    throw new Error(`Property ${key} must be unique`);
                }
            }
        }

        return doc;
    }

    async create(doc: InferSchemaType<typeof this.schema>): Promise<InferSchemaType<typeof this.schema>> {
        const validatedDoc = await this.validate(doc);
        return await invoke('plugin:mongoose|create', {
            collection: this.name,
            document: validatedDoc
        });
    }

}