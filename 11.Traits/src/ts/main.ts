import { NewsArticle } from "./news_article";

// Test by: bun run main.ts

(function main() {
	const article = new NewsArticle();

	article.headline = "Rust imports suck";
	article.author = "Joe";
	article.location = "Mama's house";
	article.content = "bla bla bla";

	console.log(article.summarize(article));
})();
