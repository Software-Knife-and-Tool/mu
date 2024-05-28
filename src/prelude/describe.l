;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(crux:intern prelude "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (prelude:format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(crux:heap-size fn)
              ,(prelude:type-of fn)
              ,(crux:vector-ref view 2)
              ,(crux:vector-ref view 0)
              ,(crux:vector-ref view 1))))
       (crux:view fn))))

(crux:intern prelude "%describe-prelude-type"
   (:lambda (value stream)
      (prelude:format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(crux:heap-size value)
         ,(prelude:type-of value)
         ,(crux:struct-ref (crux:st-vec value) 0)
         ,(crux:struct-ref (crux:st-vec value) 0)))))

(crux:intern prelude "%describe-struct"
   (:lambda (struct stream)
      (prelude:format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(crux:heap-size struct)
         ,(prelude:type-of struct)
         ,(crux:st-type struct)
         ,(crux:st-vec struct)))))

(crux:intern prelude "%describe-symbol"
   (:lambda (symbol stream)
      (:if (prelude:null symbol)
           (prelude:format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(crux:heap-size symbol)))
           (:if (prelude:keywordp symbol)
                (prelude:format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(crux:heap-size symbol) ,(crux:symbol-name symbol) ,(crux:symbol-value symbol)))
                (:if (crux:boundp symbol)
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(crux:heap-size symbol)
                            ,(crux:struct-ref view 0)
                            ,(crux:struct-ref view 1)
                            ,(crux:struct-ref view 2))))
                      (crux:view symbol))
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(crux:heap-size symbol)
                            ,(crux:struct-ref view 0)
                            ,(crux:struct-ref view 1)
                            ,(crux:struct-ref view 2))))
                      (crux:view symbol)))))))

(crux:intern prelude "%describe-fixnum"
   (:lambda (fx stream)
      (prelude:format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 61b signed integer~%    value:  ~A~%"
        `(,(crux:heap-size fx) ,(prelude:type-of fx) ,fx))))

(crux:intern prelude "%describe-char"
   (:lambda (ch stream)
      (prelude:format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character~%    value:  ~A~%"
        `(,(crux:heap-size ch) ,(prelude:type-of ch) ,ch))))

(crux:intern prelude "%describe-float"
   (:lambda (fl stream)
      (prelude:format stream
       "float: :float ~A~%    format: 32b IEEE single float~%    value:  ~A~%"
       `(,(prelude:type-of fl) ,fl))))

(crux:intern prelude "%describe-string"
   (:lambda (str stream)
      (prelude:format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(crux:heap-size str) ,(crux:struct-len str) ,str))))

(crux:intern prelude "%describe-vector"
   (:lambda (vec stream)
      (prelude:format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(crux:heap-size vec) ,(prelude:type-of vec) ,(crux:struct-type vec) ,(crux:struct-type vec) ,vec))))

(crux:intern prelude "%describe-cons"
   (:lambda (cons stream)
      (:if (prelude:dotted-pair-p cons)
           (prelude:format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(crux:heap-size cons) ,(prelude:type-of cons) ,cons))
           (prelude:format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(crux:heap-size cons) ,(prelude:type-of cons) ,(crux:length cons) ,cons)))))

(crux:intern prelude "describe"
   (:lambda (value stream)
      (crux:fix
       (:lambda (list)
          (:if (prelude:null list)
               ()
               ((:lambda (predicate fn)
                   (:if (crux:apply predicate `(,value))
                        ((:lambda ()
                            (crux:apply fn `(,value ,stream))
                            ()))
                        (crux:cdr list)))
               (crux:car (crux:car list))
               (crux:cdr (crux:car list)))))
       `(,(crux:cons prelude:functionp prelude:%describe-function)
         ,(crux:cons prelude:charp prelude:%describe-char)
         ,(crux:cons prelude:%prelude-type-p prelude:%describe-prelude-type)
         ,(crux:cons prelude:structp prelude:%describe-struct)
         ,(crux:cons prelude:symbolp prelude:%describe-symbol)
         ,(crux:cons prelude:fixnump prelude:%describe-fixnum)
         ,(crux:cons prelude:floatp prelude:%describe-float)
         ,(crux:cons prelude:stringp prelude:%describe-string)
         ,(crux:cons prelude:vectorp prelude:%describe-vector)
          ,(crux:cons prelude:consp prelude:%describe-cons)))))
