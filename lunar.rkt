#lang racket

(require gregor)

(define lunar-letters
  #("a"
    "b"
    "c"
    "d"
    "e"
    "f"
    "g"
    "h"
    "i"
    "k"
    "l"
    "m"
    "n"
    "p"
    "q"
    "r"
    "s"
    "t"
    "u"
    "A"
    "B"
    "C"
    "D"
    "E"
    "F1"
    "F2"
    "G"
    "H"
    "M"
    "N"
    "P"))

(define letter-count (vector-length lunar-letters))
(define A-index 19)
(define F1-index 24)
(define F2-index 25)

(define cycle-start 6)
(define cycle-length 59)

(define (lunar-numbers-1 month day)
  (let* ([yday (->yday (date 2001 month day))]
         [position (remainder (- yday cycle-start) cycle-length)])
    (cond
      [(< yday cycle-start) (values (+ yday 25) 30 #f)]
      [(<= position 28) (values (+ position 1) 30 #f)]
      [(= position 29) (values 30 30 1)]
      [(<= position 57) (values (- position 29) 29 #f)]
      [(= position 58) (values 29 29 30)])))

(define (lunar-numbers-2 F2 wrap-at [F1 #f])
  (define result (make-vector letter-count))
  (define values
    (let* ([start (- F2 1)]
           [stop (+ start letter-count)])
      (map (lambda (n) (+ 1 (remainder n wrap-at))) (range start stop))))
  (for ([v values]
        [i letter-count])
    (let ([index (remainder (+ i F2-index) letter-count)]) (vector-set! result index v)))
  (when F1
    (vector-set! result F1-index F1))
  result)

(define lunar-numbers (compose lunar-numbers-2 lunar-numbers-1))

(define (rubr s)
  (format "\textcolor{red}{~a}" s))

(define formatted-lunar-letters (vector-map (lambda (c) (rubr c)) lunar-letters))

(define (repeat-char c n)
  (build-string n (lambda (i) c)))

(define (tex-tables lnums)
  (string-append "\begin{tabular}{"
                 (repeat-char #\c A-index)
                 "}\n"
                 (string-join (vector->list formatted-lunar-letters) " & " #:after-last "\\\\")
                 "\end{tabular}"))
