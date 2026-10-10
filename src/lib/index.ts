export function Data<T extends object>() {
  return class {
    constructor(init: T) {
      Object.assign(this, init);
    }
  } as new (init: T) => T;
}
