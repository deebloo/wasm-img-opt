import init, { ImageOptimizer } from "./pkg/image_opt";

const file = document.getElementById("file-upload") as HTMLInputElement;

export async function main() {
  await init();

  const optimizer = ImageOptimizer.new();

  file.onchange = async () => {
    const files = file.files || [];

    let source = await files[0].arrayBuffer();

    const data = optimizer.grayscale(new Uint8Array(source));
    const blob = new Blob([data]);
    const url = URL.createObjectURL(blob);
    const img = new Image();
    img.src = url;

    console.log("data: " + data);
    console.log("url: " + url);

    document.body.appendChild(img);
  };
}
