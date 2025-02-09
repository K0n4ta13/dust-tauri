import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CodeEditor } from '@acrodata/code-editor';
import { LanguageDescription } from '@codemirror/language';
import { invoke } from '@tauri-apps/api/core';
import { Token, TokenArray } from './interfaces';

@Component({
  selector: 'app-root',
  imports: [FormsModule, CodeEditor],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'dust';
  value = `fn main() {
    println!("Hello, World!");
}
  `;
  tokens: Token[] = [];

  languages = [
    LanguageDescription.of({
      name: "Rust",
      extensions: ["rs"],
      load() {
        return import("@codemirror/lang-rust").then(m => m.rust())
      }
    })
  ]

  onFileSelected(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      const reader = new FileReader();
      reader.onload = () => {
        this.value = reader.result as string;
      };
      reader.readAsText(file);
    }
  }
}
