;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(mu:intern mu:%null-ns% "describe" (mu:make-namespace "describe"))
(mu:intern describe "+version+" "0.0.1")

(core:provide "describe" ())

;;;
;;; describe
;;;
(mu:intern describe "%describe-fn"
   (:lambda (fn)
     (:if (core:%closure-p fn)
          ((:lambda (lambda arity fn env)
             (core:eprint "" "%describe-fn:core")
             (core:eprint lambda "    lambda")
             (core:eprint arity "    arity")
             (core:eprint fn "    fn")
             (core:eprint env "    env"))
           (core:%fn-prop :lambda fn)
           (core:%fn-prop :arity fn)
           (core:%fn-prop :fn fn)
           (core:%fn-prop :env fn))
          (core:eprint (mu:view fn) "%describe-fn:mu-fn"))))

(mu:intern describe "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (core:%format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(mu:heap-size fn)
              ,(prelude:type-of fn)
              ,(mu:svref view 2)
              ,(mu:svref view 0)
              ,(mu:svref view 1))))
       (mu:view fn))))

(mu:intern describe "%describe-prelude-type"
   (:lambda (value stream)
      (core:%format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(mu:heap-size value)
         ,(prelude:type-of value)
         ,(mu:struct-ref (mu:st-vec value) 0)
         ,(mu:struct-ref (mu:st-vec value) 0)))))

(mu:intern describe "%describe-struct"
   (:lambda (struct stream)
      (core:%format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(mu:heap-size struct)
         ,(prelude:type-of struct)
         ,(mu:st-type struct)
         ,(mu:st-vec struct)))))

(mu:intern describe "%describe-symbol"
   (:lambda (symbol stream)
      (:if (core:null symbol)
           (core:%format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(mu:heap-size symbol)))
           (:if (core:keywordp symbol)
                (core:%format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(mu:heap-size symbol) ,(mu:symbol-name symbol) ,(mu:symbol-value symbol)))
                (:if (mu:boundp symbol)
                     ((:lambda (view)
                         (core:%format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(mu:heap-size symbol)
                            ,(mu:struct-ref view 0)
                            ,(mu:struct-ref view 1)
                            ,(mu:struct-ref view 2))))
                      (mu:view symbol))
                     ((:lambda (view)
                         (core:%format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(mu:heap-size symbol)
                            ,(mu:struct-ref view 0)
                            ,(mu:struct-ref view 1)
                            ,(mu:struct-ref view 2))))
                      (mu:view symbol)))))))

(mu:intern describe "%describe-fixnum"
   (:lambda (fx stream)
      (core:%format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 56b signed integer immediate~%    value:  ~A~%"
        `(,(mu:heap-size fx) ,(describe:type-of fx) ,fx))))

(mu:intern describe "%describe-char"
   (:lambda (ch stream)
      (core:%format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character immediate~%    value:  ~A~%"
        `(,(mu:heap-size ch) ,(describe:type-of ch) ,ch))))

(mu:intern describe "%describe-float"
   (:lambda (fl stream)
      (core:%format stream
       "float: :float ~A~%    format: 32b IEEE single float immediate~%    value:  ~A~%"
       `(,(describe:type-of fl) ,fl))))

(mu:intern describe "%describe-string"
   (:lambda (str stream)
      (core:%format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(mu:heap-size str) ,(mu:struct-len str) ,str))))

(mu:intern describe "%describe-vector"
   (:lambda (vec stream)
      (core:%format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(mu:heap-size vec) ,(describe:type-of vec) ,(mu:struct-type vec) ,(mu:struct-type vec) ,vec))))

(mu:intern describe "%describe-cons"
   (:lambda (cons stream)
      (:if (describe:dotted-pair-p cons)
           (core:%format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(mu:heap-size cons) ,(describe:type-of cons) ,cons))
           (core:%format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(mu:heap-size cons) ,(describe:type-of cons) ,(mu:length cons) ,cons)))))

(mu:intern describe "describe"
   (:lambda (value stream)
      (mu:fix
       (:lambda (list)
          (:if (core:null list)
               ()
               ((:lambda (predicate fn)
                   (:if (mu:apply predicate `(,value))
                        ((:lambda ()
                            (mu:apply fn `(,value ,stream))
                            ()))
                        (mu:cdr list)))
               (mu:car (mu:car list))
               (mu:cdr (mu:car list)))))
       `(,(mu:cons core:functionp describe:%describe-function)
         ,(mu:cons core:charp describe:%describe-char)
         ,(mu:cons core:%core-type-p describe:%describe-describe-type)
         ,(mu:cons core:structp describe:%describe-struct)
         ,(mu:cons core:symbolp describe:%describe-symbol)
         ,(mu:cons core:fixnump describe:%describe-fixnum)
         ,(mu:cons core:floatp describe:%describe-float)
         ,(mu:cons core:stringp describe:%describe-string)
         ,(mu:cons core:vectorp describe:%describe-vector)
          ,(mu:cons core:consp describe:%describe-cons)))))
