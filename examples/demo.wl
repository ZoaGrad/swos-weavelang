(seq
  (let xs (map f (map g input)))
  (let cleaned (normalize (normalize xs)))
  (let piped (filter (lift p) (map h cleaned)))
  (ret piped)
)
