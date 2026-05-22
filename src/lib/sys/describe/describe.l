;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(mu:intern describe "%format"
   (:lambda (fmt-string stream args)
     (mu:apply (mu:symbol-value (mu:find (mu:find-namespace "format") "%format")) `(,fmt-string ,stream ,args))))

(mu:intern describe "%function"
   (:lambda (fn stream)
     (:if (core:%closurep fn)
          ((:lambda (size lambda arity fn env)
             (describe:%format stream
                              "core function: (~A bytes) :lambda ~A :arity ~A fn: ~A :env ~A"
                              `(,size ,lambda ,arity ,fn ,env)))
           (feature/env:heap-size fn)
           (core:%fn-prop :lambda fn)
           (core:%fn-prop :arity fn)
           (describe:%function (core:%fn-prop :fn fn) stream)
           (core:%fn-prop :env fn))
          ((:lambda (view)
             (describe:%format stream
                              "mu function: (~A bytes) :func :arity ~A :body ~A"
                              `(,(feature/env:heap-size fn)
                                 ,(mu:svref view 0)
                                 ,(:if (core:fixnump (mu:svref view 1)) (mu:cons ":native" (mu:svref view 1)) (mu:cons ":list" (mu:svref view 1))))))
           (mu:view fn)))))

(mu:intern describe "%struct"
   (:lambda (struct stream)
     (describe:%format stream
                      "struct: (~A bytes) :struct :type ~A :props ~A"
                      `(,(feature/env:heap-size struct)
                         ,(mu:struct-type struct)
                         ,(mu:struct-vec struct)))))

(mu:intern describe "%symbol"
   (:lambda (symbol stream)
     (:if (core:null symbol)
          (describe:%format
           stream
           "symbol: (~A bytes) :null null :ns () :name :nil :value ()"
           `(,(feature/env:heap-size symbol)))
          (:if (core:keywordp symbol)
               (describe:%format
                stream
                "symbol: (~A bytes) :symbol keyword :ns keyword :name ~A :value: ~A"
                `(,(feature/env:heap-size symbol) ,(mu:symbol-name symbol) ,(mu:symbol-value symbol)))
               (:if (mu:boundp symbol)
                    ((:lambda (view)
                       (describe:%format
                        stream
                        "symbol: (~A bytes) :symbol symbol :ns ~A :name ~A value: ~A"
                        `(,(feature/env:heap-size symbol)
                           ,(mu:svref view 0)
                           ,(mu:svref view 1)
                           ,(mu:svref view 2))))
                     (mu:view symbol))
                    ((:lambda (view)
                       (describe:%format
                        stream
                        "symbol: (~A bytes) :symbol symbol :ns ~A :name ~A :unbound: :t"
                        `(,(feature/env:heap-size symbol)
                           ,(mu:svref view 0)
                           ,(mu:svref view 1)
                           ,(mu:svref view 2))))
                     (mu:view symbol)))))))

(mu:intern describe "%fixnum"
   (:lambda (fx stream)
     (describe:%format
      stream
      "fixnum: (~A bytes) :fixnum :format: 56b signed integer immediate :value ~A"
      `(,(feature/env:heap-size fx) ,fx))))

(mu:intern describe "%char"
   (:lambda (ch stream)
     (describe:%format
      stream
      "char: (~A bytes) :char :format 8b ASCII character immediate :value ~A"
      `(,(feature/env:heap-size ch) ,ch))))

(mu:intern describe "%float"
   (:lambda (fl stream)
     (describe:%format
      stream
      "float: (~A bytes) :float :format: 32b IEEE single float immediate :value ~A"
      `(,(feature/env:heap-size fl) ,fl))))

(mu:intern describe "%string"
   (:lambda (str stream)
     (describe:%format
      stream
      "string: (~A bytes) :vector char :format: unsigned 8b character vector :length ~A :value ~A"
      `(,(feature/env:heap-size str) ,(mu:vector-length str) ,str))))

(mu:intern describe "%vector"
   (:lambda (vec stream)
     (describe:%format
      stream
      "vector: (~A bytes) :vector :format: ~A :length ~A :elements ~A"
      `(,(feature/env:heap-size vec) ,(mu:vector-type vec) ,(mu:vector-length vec) ,vec))))

(mu:intern describe "%cons"
   (:lambda (cons stream)
     (describe:%format
      stream
      "cons: (~A bytes) :cons :length ~A :value ~A"
      `(,(feature/env:heap-size cons) ,(mu:length cons) ,cons))))

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
              `(,(mu:cons core:functionp describe:%function)
                 ,(mu:cons core:charp describe:%char)
                 ,(mu:cons core:structp describe:%struct)
                 ,(mu:cons core:symbolp describe:%symbol)
                 ,(mu:cons core:fixnump describe:%fixnum)
                 ,(mu:cons core:floatp describe:%float)
                 ,(mu:cons core:stringp describe:%string)
                 ,(mu:cons core:vectorp describe:%vector)
                 ,(mu:cons core:consp describe:%cons)))))

