# Charabia
Library used by Meilisearch to tokenize queries and documents

## Role

The tokenizerโs role is to take a sentence or phrase and split it into smaller units of language, called tokens. It finds and retrieves all the words in a string based on the languageโs particularities.

## Details

Charabia provides a simple API to segment, normalize, or tokenize (segment + normalize) a text of a specific language by detecting its Script/Language and choosing the specialized pipeline for it.

## Supported languages

**Charabia is multilingual**, featuring optimized support for:


|  Script - Language  |                           specialized segmentation                            | specialized normalization | Segmentation Performance level | Tokenization Performance level |
|---------------------|-------------------------------------------------------------------------------|---------------------------|-------------------|---|
| **Latin** - **Any** | โ [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | โ lowercase + deunicode            | ๐จ ~13MiB/sec    | ๐ง ~5MiB/sec    |
| **Chinese** - **CMN** ๐จ๐ณ | โ [jieba](https://github.com/messense/jieba-rs) | โ traditional-to-simplified conversion | ๐จ ~9MiB/sec    | ๐ง ~5MiB/sec    |
| **Hebrew** ๐ฎ๐ฑ | โ [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | โ diacritics removal  | ๐ฉ ~21MiB/sec    | ๐จ ~11MiB/sec    |
| **Japanese** ๐ฏ๐ต | โ [lindera](https://github.com/lindera-morphology/lindera) | โ | ๐ง ~3MiB/sec    | ๐ฅ ~2MiB/sec    |
| **Thai** ๐น๐ญ | โ [dictionary based](https://github.com/PyThaiNLP/nlpo3) | โ | ๐ฉ ~23MiB/sec    | ๐จ ~14MiB/sec    |

We aim to provide global language support, and your feedback helps us [move closer to that goal](https://docs.meilisearch.com/learn/advanced/language.html#improving-our-language-support). If you notice inconsistencies in your search results or the way your documents are processed, please open an issue on our [GitHub repository](https://github.com/meilisearch/charabia/issues/new/choose).

If you have a particular need that charabia does not support, please share it in the product repository by creating a [dedicated discussion](https://github.com/meilisearch/product/discussions?discussions_q=label%3Aproduct%3Acore%3Atokenizer).

### About Performance level

Performances are based on the throughput (MiB/sec) of the tokenizer (computed on a [scaleway Elastic Metal server EM-A410X-SSD](https://www.scaleway.com/en/pricing/) - CPU: Intel Xeon E5 1650 - RAM: 64 Go) using jemalloc:
- 0๏ธโฃโฌ๏ธ:  0  ->  1  MiB/sec
- 1๏ธโฃ๐ฅ:  1  ->  3  MiB/sec
- 2๏ธโฃ๐ง:  3  ->  8  MiB/sec
- 3๏ธโฃ๐จ:  8  -> 20  MiB/sec
- 4๏ธโฃ๐ฉ: 20  -> 50  MiB/sec
- 5๏ธโฃ๐ช: 50 MiB/sec or more

## Examples

#### Tokenization

```rust
use charabia::Tokenize;

let orig = "Thรฉ quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3ยฐF!";

// tokenize the text.
let mut tokens = orig.tokenize();

let token = tokens.next().unwrap();
// the lemma into the token is normalized: `Thรฉ` became `the`.
assert_eq!(token.lemma(), "the");
// token is classfied as a word
assert!(token.is_word());

let token = tokens.next().unwrap();
assert_eq!(token.lemma(), " ");
// token is classfied as a separator
assert!(token.is_separator());
```

#### Segmentation

```rust
use charabia::Segment;

let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3ยฐF!";

// segment the text.
let mut segments = orig.segment_str();

assert_eq!(segments.next(), Some("The"));
assert_eq!(segments.next(), Some(" "));
assert_eq!(segments.next(), Some("quick"));
```
