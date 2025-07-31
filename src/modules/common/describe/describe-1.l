;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
#|
(mu:intern common "%describe-core-fn"
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
|#

(mu:intern common "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (core:%format stream
            "function: (~A bytes) :func ~A~%    arity: ~A~%    body:  ~A~%"
            `(,(%env%:heap-size fn)
              ,(core:type-of fn)
              ,(mu:svref view 0)
              ,(mu:svref view 1))))
       (mu:view fn))))

(mu:intern common "%describe-common-type"
   (:lambda (value stream)
      (core:%format stream
       "core-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(%env%:heap-size value)
         ,(common:type-of value)
         ,(mu:struct-ref (mu:st-vec value) 0)
         ,(mu:struct-ref (mu:st-vec value) 0)))))

(mu:intern common "%describe-struct"
   (:lambda (struct stream)
      (core:%format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(%env%:heap-size struct)
         ,(core:type-of struct)
         ,(mu:st-type struct)
         ,(mu:st-vec struct)))))

(mu:intern common "%describe-symbol"
   (:lambda (symbol stream)
      (:if (core:null symbol)
           (core:%format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(%env%:heap-size symbol)))
           (:if (core:keywordp symbol)
                (core:%format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(%env%:heap-size symbol) ,(mu:symbol-name symbol) ,(mu:symbol-value symbol)))
                (:if (mu:boundp symbol)
                     ((:lambda (view)
                         (core:%format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(%env%:heap-size symbol)
                            ,(mu:struct-ref view 0)
                            ,(mu:struct-ref view 1)
                            ,(mu:struct-ref view 2))))
                      (mu:view symbol))
                     ((:lambda (view)
                         (core:%format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(%env%:heap-size symbol)
                            ,(mu:struct-ref view 0)
                            ,(mu:struct-ref view 1)
                            ,(mu:struct-ref view 2))))
                      (mu:view symbol)))))))

(mu:intern common "%describe-fixnum"
   (:lambda (fx stream)
      (core:%format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 56b signed integer immediate~%    value:  ~A~%"
        `(,(%env%:heap-size fx) ,(common:type-of fx) ,fx))))

(mu:intern common "%describe-char"
   (:lambda (ch stream)
      (core:%format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character immediate~%    value:  ~A~%"
        `(,(%env%:heap-size ch) ,(common:type-of ch) ,ch))))

(mu:intern common "%describe-float"
   (:lambda (fl stream)
      (core:%format stream
       "float: :float ~A~%    format: 32b IEEE single float immediate~%    value:  ~A~%"
       `(,(common:type-of fl) ,fl))))

(mu:intern common "%describe-string"
   (:lambda (str stream)
      (core:%format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(%env%:heap-size str) ,(mu:struct-len str) ,str))))

(mu:intern common "%describe-vector"
   (:lambda (vec stream)
      (core:%format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(%env%:heap-size vec) ,(common:type-of vec) ,(mu:struct-type vec) ,(mu:struct-type vec) ,vec))))

(mu:intern common "%describe-cons"
   (:lambda (cons stream)
      (:if (common:dotted-pair-p cons)
           (core:%format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(%env%:heap-size cons) ,(common:type-of cons) ,cons))
           (core:%format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(%env%:heap-size cons) ,(common:type-of cons) ,(mu:length cons) ,cons)))))

(mu:intern common "describe"
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
       `(,(mu:cons core:functionp common:%describe-function)
          ,(mu:cons core:charp common:%describe-char)
          ,(mu:cons core:%core-type-p common:%describe-common-type)
          ,(mu:cons core:structp common:%describe-struct)
          ,(mu:cons core:symbolp common:%describe-symbol)
          ,(mu:cons core:fixnump common:%describe-fixnum)
          ,(mu:cons core:floatp common:%describe-float)
          ,(mu:cons core:stringp common:%describe-string)
          ,(mu:cons core:vectorp common:%describe-vector)
          ,(mu:cons core:consp common:%describe-cons)))))
