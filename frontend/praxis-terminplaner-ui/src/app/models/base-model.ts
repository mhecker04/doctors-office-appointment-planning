

export abstract class BaseModel<TPrimaryKey> {

    abstract getPrimaryKey(): TPrimaryKey | undefined;
    abstract setPrimaryKey(primaryKey: TPrimaryKey): void;

}
