(when a
b)

(when)


(when
a
b)

(when a b
c)

(when
    a)

(when @a b
      c)

(when #a b
  c)

(when
    a
  b)

(when
    a
  b
  )

(when
    a b
    c)

(condp ^:b
    :a
    :b
  :c
  d)

(when ^:b #a ^:b b
  d)

(when a b
      c)

(condp a
    b d
    c)

(when a b c d e
      f)
