;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(lib:intern :prelude "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (prelude:format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(lib:heap-size fn)
              ,(prelude:type-of fn)
              ,(lib:sv-ref view 2)
              ,(lib:sv-ref view 0)
              ,(lib:sv-ref view 1))))
       (lib:view fn))))

(lib:intern :prelude "%describe-prelude-type"
   (:lambda (value stream)
      (prelude:format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(lib:heap-size value)
         ,(prelude:type-of value)
         ,(lib:sv-ref (lib:st-vec value) 0)
         ,(lib:sv-ref (lib:st-vec value) 0)))))

(lib:intern :prelude "%describe-struct"
   (:lambda (struct stream)
      (prelude:format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(lib:heap-size struct)
         ,(prelude:type-of struct)
         ,(lib:st-type struct)
         ,(lib:st-vec struct)))))

(lib:intern :prelude "%describe-symbol"
   (:lambda (symbol stream)
      (:if (prelude:null symbol)
           (prelude:format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(lib:heap-size symbol)))
           (:if (prelude:keywordp symbol)
                (prelude:format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(lib:heap-size symbol) ,(lib:sy-name symbol) ,(lib:sy-val symbol)))
                (:if (lib:boundp symbol)
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(lib:heap-size symbol)
                            ,(lib:sv-ref view 0)
                            ,(lib:sv-ref view 1)
                            ,(lib:sv-ref view 2))))
                      (lib:view symbol))
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(lib:heap-size symbol)
                            ,(lib:sv-ref view 0)
                            ,(lib:sv-ref view 1)
                            ,(lib:sv-ref view 2))))
                      (lib:view symbol)))))))

(lib:intern :prelude "%describe-fixnum"
   (:lambda (fx stream)
      (prelude:format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 61b signed integer~%    value:  ~A~%"
        `(,(lib:heap-size fx) ,(prelude:type-of fx) ,fx))))

(lib:intern :prelude "%describe-char"
   (:lambda (ch stream)
      (prelude:format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character~%    value:  ~A~%"
        `(,(lib:heap-size ch) ,(prelude:type-of ch) ,ch))))

(lib:intern :prelude "%describe-float"
   (:lambda (fl stream)
      (prelude:format stream
       "float: :float ~A~%    format: 32b IEEE single float~%    value:  ~A~%"
       `(,(prelude:type-of fl) ,fl))))

(lib:intern :prelude "%describe-string"
   (:lambda (str stream)
      (prelude:format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(lib:heap-size str) ,(lib:sv-len str) ,str))))

(lib:intern :prelude "%describe-vector"
   (:lambda (vec stream)
      (prelude:format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(lib:heap-size vec) ,(prelude:type-of vec) ,(lib:sv-type vec) ,(lib:sv-type vec) ,vec))))

(lib:intern :prelude "%describe-cons"
   (:lambda (cons stream)
      (:if (prelude:dotted-pair-p cons)
           (prelude:format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(lib:heap-size cons) ,(prelude:type-of cons) ,cons))
           (prelude:format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(lib:heap-size cons) ,(prelude:type-of cons) ,(lib:length cons) ,cons)))))

(lib:intern :prelude "describe"
   (:lambda (value stream)
      (lib:fix
       (:lambda (list)
          (:if (prelude:null list)
               ()
               ((:lambda (predicate fn)
                   (:if (lib:apply predicate `(,value))
                        ((:lambda ()
                            (lib:apply fn `(,value ,stream))
                            ()))
                        (lib:cdr list)))
               (lib:car (lib:car list))
               (lib:cdr (lib:car list)))))
       `(,(lib:cons prelude:functionp prelude:%describe-function)
         ,(lib:cons prelude:charp prelude:%describe-char)
         ,(lib:cons prelude:%prelude-type-p prelude:%describe-prelude-type)
         ,(lib:cons prelude:structp prelude:%describe-struct)
         ,(lib:cons prelude:symbolp prelude:%describe-symbol)
         ,(lib:cons prelude:fixnump prelude:%describe-fixnum)
         ,(lib:cons prelude:floatp prelude:%describe-float)
         ,(lib:cons prelude:stringp prelude:%describe-string)
         ,(lib:cons prelude:vectorp prelude:%describe-vector)
          ,(lib:cons prelude:consp prelude:%describe-cons)))))
