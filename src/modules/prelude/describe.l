;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;

;;;
;;; describe
;;;
(mu:intern prelude "%describe-fn"
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

(mu:intern prelude "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (prelude:format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(env:heap-size fn)
              ,(prelude:type-of fn)
              ,(mu:svref view 2)
              ,(mu:svref view 0)
              ,(mu:svref view 1))))
       (mu:view fn))))

(mu:intern prelude "%describe-prelude-type"
   (:lambda (value stream)
      (prelude:format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(env:heap-size value)
         ,(prelude:type-of value)
         ,(mu:struct-ref (mu:st-vec value) 0)
         ,(mu:struct-ref (mu:st-vec value) 0)))))

(mu:intern prelude "%describe-struct"
   (:lambda (struct stream)
      (prelude:format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(env:heap-size struct)
         ,(prelude:type-of struct)
         ,(mu:st-type struct)
         ,(mu:st-vec struct)))))

(mu:intern prelude "%describe-symbol"
   (:lambda (symbol stream)
      (:if (prelude:null symbol)
           (prelude:format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(env:heap-size symbol)))
           (:if (prelude:keywordp symbol)
                (prelude:format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(env:heap-size symbol) ,(mu:symbol-name symbol) ,(mu:symbol-value symbol)))
                (:if (mu:boundp symbol)
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(env:heap-size symbol)
                            ,(mu:struct-ref view 0)
                            ,(mu:struct-ref view 1)
                            ,(mu:struct-ref view 2))))
                      (mu:view symbol))
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(env:heap-size symbol)
                            ,(mu:struct-ref view 0)
                            ,(mu:struct-ref view 1)
                            ,(mu:struct-ref view 2))))
                      (mu:view symbol)))))))

(mu:intern prelude "%describe-fixnum"
   (:lambda (fx stream)
      (prelude:format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 56b signed integer immediate~%    value:  ~A~%"
        `(,(env:heap-size fx) ,(prelude:type-of fx) ,fx))))

(mu:intern prelude "%describe-char"
   (:lambda (ch stream)
      (prelude:format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character immediate~%    value:  ~A~%"
        `(,(env:heap-size ch) ,(prelude:type-of ch) ,ch))))

(mu:intern prelude "%describe-float"
   (:lambda (fl stream)
      (prelude:format stream
       "float: :float ~A~%    format: 32b IEEE single float immediate~%    value:  ~A~%"
       `(,(prelude:type-of fl) ,fl))))

(mu:intern prelude "%describe-string"
   (:lambda (str stream)
      (prelude:format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(env:heap-size str) ,(mu:struct-len str) ,str))))

(mu:intern prelude "%describe-vector"
   (:lambda (vec stream)
      (prelude:format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(env:heap-size vec) ,(prelude:type-of vec) ,(mu:struct-type vec) ,(mu:struct-type vec) ,vec))))

(mu:intern prelude "%describe-cons"
   (:lambda (cons stream)
      (:if (prelude:dotted-pair-p cons)
           (prelude:format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(env:heap-size cons) ,(prelude:type-of cons) ,cons))
           (prelude:format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(env:heap-size cons) ,(prelude:type-of cons) ,(mu:length cons) ,cons)))))

(mu:intern prelude "describe"
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
       `(,(mu:cons core:functionp prelude:%describe-function)
         ,(mu:cons core:charp prelude:%describe-char)
         ,(mu:cons core:%core-type-p prelude:%describe-prelude-type)
         ,(mu:cons core:structp prelude:%describe-struct)
         ,(mu:cons core:symbolp prelude:%describe-symbol)
         ,(mu:cons core:fixnump prelude:%describe-fixnum)
         ,(mu:cons core:floatp prelude:%describe-float)
         ,(mu:cons core:stringp prelude:%describe-string)
         ,(mu:cons core:vectorp prelude:%describe-vector)
          ,(mu:cons core:consp prelude:%describe-cons)))))

(defun describe (obj)
  (describe-function (fn)
           (let ((desc (core:fn-lambda-desc fn)))
             (format t "   nreq: ~A~%" (core::fn-nreqs fn))
             (format t " lambda: ~A~%" (core::fn-form fn))
             (cond
               ((null desc) (format t "has no lambda extension~%"))
               (t
                (format t "closure: ~A~%" (core::lambda-closure desc))
                (format t "    env: ~A~%" (core::lambda-env desc))
                (format t " macrop: ~A~%" (core::lambda-macrop desc))
                (format t "   reqs: ~A~%" (core::lambda-req-symbols desc))
                (format t "   rest: ~A~%" (core::lambda-rest-symbol desc))))
             fn))
         (describe-symbol (symbol)
           (errorp-unless symbolp symbol "describe-symbol: not a symbol")
           (format t "is a keyword: ~A~%" (keywordp symbol))
           (format t "        name: ~A~%" (symbol-name symbol))
           (format t "          ns: ~A~%" (symbol-ns symbol))
           (if (boundp symbol)
               (format t "  value: ~A~%" (symbol-value symbol))
               (format t "  value: is unbound~%"))
           symbol)
         (describe-vector (vector)
           (errorp-unless vectorp vector "describe-vector: not a vector")
           (format t "  type: ~A~%" (vector-type vector))
           (format t "length: ~A~%" (length vector))
           vector))
    (cond
      ((consp obj) (format t "is a cons: length ~A ~A~%" (length obj) obj))
      ((functionp obj) (format t "is a function: ~A~%" obj) (describe-function obj))
      ((fixnump obj) (format t "is a fixnum: ~A~%" obj))
      ((stringp obj) (format t "is a string byte vector: ~A~%" obj))
      ((symbolp obj) (format t "is a symbol: ~A~%" obj) (describe-symbol obj))
      ((vectorp obj) (format t "is a vector: ~A~%" obj) (escribe-vector obj))
      (t (format t "is undescribed: ~A~%" obj)))))

