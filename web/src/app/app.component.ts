import { Component, ElementRef, ViewChild } from '@angular/core';
import { fileOpen, FileWithHandle, supported } from 'browser-fs-access';
import * as wasm from 'image-resizer';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'image-resizer';

  @ViewChild('image') image!: ElementRef;

  fileName?: string;
  data?: Uint8Array;

  originalWidth: number = 0;
  originalHeight: number = 0;

  width: number | null = null;
  height: number | null = null;

  async open() {
    if (!supported) {
      alert('Sorry, your browser not support file system API.');
      return;
    }

    try {
      let blobs = await fileOpen({
        description: 'Image File',
        mimeTypes: ['image/png', 'image/gif'],
        extensions: ['.png', '.gif']
      });

      this.fileName = blobs.name;
      this.setPreview(blobs);

      let buffer = await new Promise<ArrayBuffer>((resolve, reject) => {
        let reader = new FileReader();
        reader.onload = () => {
          resolve(reader.result as ArrayBuffer);
        };
        reader.onerror = reject;
        reader.readAsArrayBuffer(blobs);
      });

      this.data = new Uint8Array(buffer);

      let size = wasm.get_size(this.data);
      this.originalWidth = size[0];
      this.originalHeight = size[1];
    } catch (err) {
      if ((err as Error)?.name == 'AbortError') {
        return;
      }

      alert('Open file failed.');
    }
  }

  save() {
    if (!this.data || !this.fileName) {
      alert('Please open image file first');
      return;
    }

    if (this.width == null && this.height == null) {
      alert('Please set width or height');
      return;
    }

    let width = this.width;
    let height = this.height;

    if (width == null) {
      width = this.originalWidth * (height as number) / this.originalHeight;
    }

    if (height == null) {
      height = this.originalHeight * (width as number) / this.originalWidth;
    }

    var result = wasm.resize(this.data, width, height);
    var blob = new Blob([result], { type: 'octet/stream' });
    var url = URL.createObjectURL(blob);
    var a = document.createElement('a');
    a.style.display = 'none';
    a.href = url;
    a.download = this.fileName;
    document.body.appendChild(a);

    a.click();

    URL.revokeObjectURL(url);
    a.remove();
  }

  setPreview(blobs: FileWithHandle) {
    let img = this.getImage();
    img.src = URL.createObjectURL(blobs);
  }

  getImage() {
    return this.image.nativeElement as HTMLImageElement;
  }

  onError() {
    let img = this.getImage();
    URL.revokeObjectURL(img.src);
  }
}
