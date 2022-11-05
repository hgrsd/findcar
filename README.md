# findcar

A cli that lets you search, sort, aggregate and filter results from multiple car search engines. Because of course you want to search for cars from the command line.

# Usage

```
Usage: findcar [OPTIONS]

Options:
      --make <MAKE>
          Optional, make of the car to search for
      --model <MODEL>
          Optional, model of the car to search for
      --min-year <MIN_YEAR>
          Optional, minimum year of registration
      --max-year <MAX_YEAR>
          Optional, maximum year of registration
      --min-kms <MIN_KMS>
          Optional, minimum kms
      --max-kms <MAX_KMS>
          Optional, maximum kms
      --min-price <MIN_PRICE>
          Optional, minimum price
      --max-price <MAX_PRICE>
          Optional, maximum price
      --sort-by <SORT_BY>
          Optional, value to sort by. Options are: price, year, mileage
      --sort-order <SORT_ORDER>
          Optional, sort order. Options are ASC, DESC. 
          If not specified, but a sort-by value *is*, then ASC will be used by default
      --limit <LIMIT>
          Optional, maximum number of results to return
      --emitter <EMITTER>
          Optional, emitter for the results. Options are: json, text. Default is text
      --search-engine <SEARCH_ENGINE>
          Optional, search engine to use. Options are donedeal_ie, carzone_ie. 
          Default is to use all available engines. 
          Example: ./findcar [other opts] --search-engine carzone_ie --search-engine donedeal_ie
  -h, --help
          Print help information
  -V, --version
          Print version information
```

# Architecture

The goal of `findcar` is to be easily extensible with further car search engines. Every search engine is expected to implement the `Searcher` trait, after which it can be plugged into the main engine as one of the searchers.

# Searchers

This project comes with two searchers:

* [donedeal.ie](src/search/donedeal_ie.rs)
* [carzone.ie](src/search/carzone_ie.rs)


# Contributions

Any contributions are very much welcome. Please feel free to suggest/implement new search engines or new functionaliy and I would be very happy to review and discuss!
