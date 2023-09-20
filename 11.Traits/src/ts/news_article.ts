import { Summary } from "./summary";

export class NewsArticle implements Summary {
	public headline: string;
	public location: string;
	public author: string;
	public content: string;

	public summarize(self: NewsArticle) {
		return `${self.headline}, by ${self.author} (${self.location})`;
	}
}
