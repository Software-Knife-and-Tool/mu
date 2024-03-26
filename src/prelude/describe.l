;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(libcore:intern :prelude "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (prelude:format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(libcore:hp-size fn)
              ,(prelude:type-of fn)
              ,(libcore:sv-ref view 2)
              ,(libcore:sv-ref view 0)
              ,(libcore:sv-ref view 1))))
       (libcore:view fn))))

(libcore:intern :prelude "%describe-prelude-type"
   (:lambda (value stream)
      (prelude:format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(libcore:hp-size value)
         ,(prelude:type-of value)
         ,(libcore:sv-ref (libcore:st-vec value) 0)
         ,(libcore:sv-ref (libcore:st-vec value) 0)))))

(libcore:intern :prelude "%describe-struct"
   (:lambda (struct stream)
      (prelude:format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(libcore:hp-size struct)
         ,(prelude:type-of struct)
         ,(libcore:st-type struct)
         ,(libcore:st-vec struct)))))

(libcore:intern :prelude "%describe-symbol"
   (:lambda (symbol stream)
      (:if (prelude:null symbol)
           (prelude:format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(libcore:hp-size symbol)))
           (:if (prelude:keywordp symbol)
                (prelude:format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(libcore:hp-size symbol) ,(libcore:sy-name symbol) ,(libcore:sy-val symbol)))
                (:if (libcore:boundp symbol)
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(libcore:hp-size symbol)
                            ,(libcore:sv-ref view 0)
                            ,(libcore:sv-ref view 1)
                            ,(libcore:sv-ref view 2))))
                      (libcore:view symbol))
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(libcore:hp-size symbol)
                            ,(libcore:sv-ref view 0)
                            ,(libcore:sv-ref view 1)
                            ,(libcore:sv-ref view 2))))
                      (libcore:view symbol)))))))

(libcore:intern :prelude "%describe-fixnum"
   (:lambda (fx stream)
      (prelude:format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 61b signed integer~%    value:  ~A~%"
        `(,(libcore:hp-size fx) ,(prelude:type-of fx) ,fx))))

(libcore:intern :prelude "%describe-char"
   (:lambda (ch stream)
      (prelude:format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character~%    value:  ~A~%"
        `(,(libcore:hp-size ch) ,(prelude:type-of ch) ,ch))))

(libcore:intern :prelude "%describe-float"
   (:lambda (fl stream)
      (prelude:format stream
       "float: :float ~A~%    format: 32b IEEE single float~%    value:  ~A~%"
       `(,(prelude:type-of fl) ,fl))))

(libcore:intern :prelude "%describe-string"
   (:lambda (str stream)
      (prelude:format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(libcore:hp-size str) ,(libcore:sv-len str) ,str))))

(libcore:intern :prelude "%describe-vector"
   (:lambda (vec stream)
      (prelude:format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(libcore:hp-size vec) ,(prelude:type-of vec) ,(libcore:sv-type vec) ,(libcore:sv-type vec) ,vec))))

(libcore:intern :prelude "%describe-cons"
   (:lambda (cons stream)
      (:if (prelude:dotted-pair-p cons)
           (prelude:format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(libcore:hp-size cons) ,(prelude:type-of cons) ,cons))
           (prelude:format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(libcore:hp-size cons) ,(prelude:type-of cons) ,(libcore:length cons) ,cons)))))

(libcore:intern :prelude "describe"
   (:lambda (value stream)
      (libcore:fix
       (:lambda (list)
          (:if (prelude:null list)
               ()
               ((:lambda (predicate fn)
                   (:if (libcore:apply predicate `(,value))
                        ((:lambda ()
                            (libcore:apply fn `(,value ,stream))
                            ()))
                        (libcore:cdr list)))
               (libcore:car (libcore:car list))
               (libcore:cdr (libcore:car list)))))
       `(,(libcore:cons prelude:functionp prelude:%describe-function)
         ,(libcore:cons prelude:charp prelude:%describe-char)
         ,(libcore:cons prelude:%prelude-type-p prelude:%describe-prelude-type)
         ,(libcore:cons prelude:structp prelude:%describe-struct)
         ,(libcore:cons prelude:symbolp prelude:%describe-symbol)
         ,(libcore:cons prelude:fixnump prelude:%describe-fixnum)
         ,(libcore:cons prelude:floatp prelude:%describe-float)
         ,(libcore:cons prelude:stringp prelude:%describe-string)
         ,(libcore:cons prelude:vectorp prelude:%describe-vector)
          ,(libcore:cons prelude:consp prelude:%describe-cons)))))
