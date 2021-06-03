export interface Action {
  type: "BLUR" | "GRAYSCALE" | "INVERT" | "THUMBNAIL";
  payload: Uint8Array;
}

export interface Response {
  payload: Uint8Array;
}
