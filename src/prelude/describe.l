;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(core:intern prelude "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (prelude:format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(core:heap-size fn)
              ,(prelude:type-of fn)
              ,(core:vector-ref view 2)
              ,(core:vector-ref view 0)
              ,(core:vector-ref view 1))))
       (core:view fn))))

(core:intern prelude "%describe-prelude-type"
   (:lambda (value stream)
      (prelude:format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(core:heap-size value)
         ,(prelude:type-of value)
         ,(core:struct-ref (core:st-vec value) 0)
         ,(core:struct-ref (core:st-vec value) 0)))))

(core:intern prelude "%describe-struct"
   (:lambda (struct stream)
      (prelude:format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(core:heap-size struct)
         ,(prelude:type-of struct)
         ,(core:st-type struct)
         ,(core:st-vec struct)))))

(core:intern prelude "%describe-symbol"
   (:lambda (symbol stream)
      (:if (prelude:null symbol)
           (prelude:format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(core:heap-size symbol)))
           (:if (prelude:keywordp symbol)
                (prelude:format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(core:heap-size symbol) ,(core:symbol-name symbol) ,(core:symbol-value symbol)))
                (:if (core:boundp symbol)
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(core:heap-size symbol)
                            ,(core:struct-ref view 0)
                            ,(core:struct-ref view 1)
                            ,(core:struct-ref view 2))))
                      (core:view symbol))
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(core:heap-size symbol)
                            ,(core:struct-ref view 0)
                            ,(core:struct-ref view 1)
                            ,(core:struct-ref view 2))))
                      (core:view symbol)))))))

(core:intern prelude "%describe-fixnum"
   (:lambda (fx stream)
      (prelude:format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 61b signed integer~%    value:  ~A~%"
        `(,(core:heap-size fx) ,(prelude:type-of fx) ,fx))))

(core:intern prelude "%describe-char"
   (:lambda (ch stream)
      (prelude:format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character~%    value:  ~A~%"
        `(,(core:heap-size ch) ,(prelude:type-of ch) ,ch))))

(core:intern prelude "%describe-float"
   (:lambda (fl stream)
      (prelude:format stream
       "float: :float ~A~%    format: 32b IEEE single float~%    value:  ~A~%"
       `(,(prelude:type-of fl) ,fl))))

(core:intern prelude "%describe-string"
   (:lambda (str stream)
      (prelude:format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(core:heap-size str) ,(core:struct-len str) ,str))))

(core:intern prelude "%describe-vector"
   (:lambda (vec stream)
      (prelude:format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(core:heap-size vec) ,(prelude:type-of vec) ,(core:struct-type vec) ,(core:struct-type vec) ,vec))))

(core:intern prelude "%describe-cons"
   (:lambda (cons stream)
      (:if (prelude:dotted-pair-p cons)
           (prelude:format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(core:heap-size cons) ,(prelude:type-of cons) ,cons))
           (prelude:format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(core:heap-size cons) ,(prelude:type-of cons) ,(core:length cons) ,cons)))))

(core:intern prelude "describe"
   (:lambda (value stream)
      (core:fix
       (:lambda (list)
          (:if (prelude:null list)
               ()
               ((:lambda (predicate fn)
                   (:if (core:apply predicate `(,value))
                        ((:lambda ()
                            (core:apply fn `(,value ,stream))
                            ()))
                        (core:cdr list)))
               (core:car (core:car list))
               (core:cdr (core:car list)))))
       `(,(core:cons prelude:functionp prelude:%describe-function)
         ,(core:cons prelude:charp prelude:%describe-char)
         ,(core:cons prelude:%prelude-type-p prelude:%describe-prelude-type)
         ,(core:cons prelude:structp prelude:%describe-struct)
         ,(core:cons prelude:symbolp prelude:%describe-symbol)
         ,(core:cons prelude:fixnump prelude:%describe-fixnum)
         ,(core:cons prelude:floatp prelude:%describe-float)
         ,(core:cons prelude:stringp prelude:%describe-string)
         ,(core:cons prelude:vectorp prelude:%describe-vector)
          ,(core:cons prelude:consp prelude:%describe-cons)))))
