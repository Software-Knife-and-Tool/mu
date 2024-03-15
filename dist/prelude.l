;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; namespace
;;;
(mu:make-ns :prelude)

;;;
;;; types
;;;
(mu:intern :prelude "%init-ns"
   (:lambda ()
      ;;; function
      (prelude:define-type "funct"
          '((:lambda  . :struct)
            (:arity   . :fixnum)
            (:fn      . :func)
            (:env     . :listp)))

      ;;; lambda
      (prelude:define-type "lambda"
          '((:bound  . :listp)
            (:rest   . :symbolp)
            (:lexenv . :listp)))

      ;;; exception
      (prelude:define-type "except"
          '((:cond   . :symbolp)
            (:value . :t)
            (:source . :symbolp)
            (:reason . :string)
            (:env    . :listp)))

      ;;; namespace
      (prelude:define-type "namespc"
          '((:name    . :string)
            (:intern  . :keyword)
            (:extern  . :keyword)
            (:ch-mac  . :keyword)
            (:sy-mac  . :keyword)
            (:uses    . :listp)))

      ;;; vector
      (prelude:define-type "vector"
          '((:base    . :vector)
            (:disp    . :listp)
            (:bitvec  . :listp)))

      (mu:intern :prelude "%char-macro-ns%" (prelude:%gen-anon-ns))
      (mu:intern :prelude "%macro-ns%" (prelude:%gen-anon-ns))
      (mu:intern :prelude "%symbol-macro-ns%" (prelude:%gen-anon-ns))))

;;;
;;; predicates
;;;
(mu:intern :prelude "not" (:lambda (value) (prelude:null value)))
(mu:intern :prelude "atom" (:lambda (value) (prelude:not (prelude:consp value))))

(mu:intern :prelude "constantp"
   (:lambda (value)
      (:if (prelude:%orf (prelude:null value) (prelude:keywordp value))
           :t
           (:if (prelude:consp value)
                (mu:eq :quote (mu:car value))
                (prelude:not (prelude:symbolp value))))))

(mu:intern :prelude "numberp"
  (:lambda (n)
     (:if (prelude:fixnump n)
          :t
          (prelude:floatp n))))

(mu:intern :prelude "zerop"
   (:lambda (n)
     (:if (prelude:numberp n)
          (:if (mu:eq 0 n)
               :t
               (mu:eq 0.0 n))
          (prelude:raise n 'prelude:zerop "not a number"))))

(mu:intern :prelude "minusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (mu:fl-lt n 0.0)
                (mu:fx-lt n 0))
           (prelude:raise n 'prelude:minusp "not a number"))))

(mu:intern :prelude "plusp"
   (:lambda (n)
      (:if (prelude:numberp n)
           (:if (prelude:floatp n)
                (mu:fl-lt 0.0 n)
                (mu:fx-lt 0 n))
           (prelude:raise n 'prelude:plusp "not a number"))))

;;;
;;; system classes
;;;
(mu:intern :prelude "charp" (:lambda (t) (mu:eq :char (mu:type-of t))))
(mu:intern :prelude "fixnump" (:lambda (t) (mu:eq :fixnum (mu:type-of t))))
(mu:intern :prelude "floatp" (:lambda (t) (mu:eq :float (mu:type-of t))))
(mu:intern :prelude "structp" (:lambda (t) (mu:eq :struct (mu:type-of t))))

;;;
;;; syntactic sugar
;;;
(mu:intern :prelude "%orf"
   (:lambda (arg arg1)
     (:if arg arg arg1)))

(mu:intern :prelude "%andf"
   (:lambda (arg arg1)
     (:if arg arg1 ())))

;;;
;;; apply
;;;
(mu:intern :prelude "apply"
   (:lambda (function arg-list)
     (:if (prelude:functionp function)
          (:if (prelude:%prelude-function-p function)
               (prelude:%funct-apply function arg-list)
               (mu:apply function arg-list))
          (prelude:raise function 'prelude:apply "not a function designator"))))

;;;
;;; namespace components
;;;
(mu:intern :prelude "%gen-anon-ns"
   (:lambda ()
      (mu:make-ns (prelude:genkeyword #\%))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; quasiquote
;;;
(mu:intern :prelude "%list"
   (:lambda (form)
     `(,form)))

(mu:intern :prelude "%qq-func-map"
   (:lambda (key map)
     (mu:cdr (prelude:assoc key map))))

(mu:intern :prelude "%qq-read-list"
   (:lambda (stream)
     ((:lambda (syntax-map)
         (mu:cdr
          (mu:fix
           (:lambda (loop)
             ((:lambda (eol expansion)
                (:if eol
                     loop
                     ((:lambda (syntax)
                        (prelude:warn syntax "prelude:%qq-read-list")
                        ((:lambda (type expr)
                           (:if (prelude:null syntax)
                                (prelude:error stream "qquote: early end of file~%" ())
                                ((:lambda (type-fn)
                                   (mu:apply type-fn `(,expr ,expansion)))
                                 (prelude:%qq-func-map type syntax-map))))
                         (mu:car syntax)
                         (mu:cdr syntax)))
                      (prelude:%qq-read stream))))
              (mu:car loop)
              (mu:cdr loop)))
           ())))
      `(,(mu:cons :comma   (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:l-comma ,expr))))))
         ,(mu:cons :comma@ (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:l-coma@ ,expr))))))
         ,(mu:cons :atom   (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:l-atom ,expr))))))
         ,(mu:cons :list   (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion (prelude:%list `(:l-list ,expr))))))
         ,(mu:cons :list-  (:lambda (expr expansion) (mu:cons :t expansion)))
         ,(mu:cons :quasi  (:lambda (expr expansion)
                             (mu:cons () (mu:append expansion `(,(prelude:%qq-parse stream))))))))))

(mu:intern :prelude "%qq-read"
   (:lambda (stream)
     ((:lambda (ch syntax-map)
        (:if (prelude:null ch)
             (prelude:error stream "backquote: early end of file~%" ())
             ((:lambda (type-map)
                (:if (prelude:null type-map)
                     `(:atom ,(prelude:%read-dispatch ch stream))
                     (mu:apply (mu:cdr type-map) ())))
                (prelude:assoc ch syntax-map))))
        (prelude:%read-consume-ws stream)
        `(,(mu:cons #\( (:lambda () `(:list ,(prelude:%qq-read-list stream))))
           ,(mu:cons #\) (:lambda () `(:list-)))
           ,(mu:cons #\, (:lambda ()
                          ((:lambda (ch)
                             (:if (prelude:null ch)
                                  (prelude:error stream "backquote: early end of file~%" ())
                                  (:if (mu:eq #\@ ch)
                                       `(:comma@ ,(prelude:read stream () ()))
                                       ((:lambda ()
                                          (prelude:unread-char ch stream)
                                          `(:atom ,(prelude:read stream () ())))))))
                             (prelude:read-char stream () ()))))
           ,(mu:cons #\` (:lambda () `(:quasi ,(prelude:read stream () ()))))))))

(mu:intern :prelude "%qq-compile"
   (:lambda (qq-expr)
     (prelude:warn qq-expr "%qq-compile")
     ((:lambda (type expr compiler-map)
        ((:lambda (type-fn)
           (prelude:warn type-fn "type function")
           (:if (prelude:null type-fn)
                (prelude:error type "backquote: unmapped type ~A~%" `(,type))
                (mu:apply type-fn `(,expr))))
         (prelude:%qq-func-map type compiler-map)))
      (mu:nth 0 qq-expr)
      (mu:nth 1 qq-expr)
      `(,(mu:cons :l-comma  (:lambda (expr) `(mu:cons ,expr ())))
         ,(mu:cons :l-coma@ (:lambda (expr) expr))
         ,(mu:cons :l-list  (:lambda (expr)
                              ((:lambda (loop)
                                 (mu:apply loop `(,loop ,expr)))
                               (:lambda (loop list)
                                 (:if (prelude:null list)
                                      list
                                      `(mu:append
                                        ,(prelude:%qq-compile (mu:car list))
                                        ,(mu:apply loop `(,loop ,(mu:cdr list)))))))))
         ,(mu:cons :l-atom  (:lambda (expr)
                              (prelude:warn expr "list atom")
                              `(mu:cons `(:quote ,expr) ())))
         ,(mu:cons :list    (:lambda (expr)
                              ((:lambda (loop)
                                 (mu:apply loop `(,loop ,expr)))
                               (:lambda (loop list)
                                 (:if (prelude:null list)
                                      list
                                      `(mu:append
                                        ,(prelude:%qq-compile (mu:car list))
                                        ,(mu:apply loop `(,loop ,(mu:cdr list)))))))))
         ,(mu:cons :atom    (:lambda (expr)
                              (prelude:warn expr ":atom")
                              `(:quote ,expr)))
         ,(mu:cons :quasi   (:lambda (expr)
                              ((:lambda (type)
                                 (:if (mu:eq :form type)
                                      (mu:cdr expr)
                                      (:if (mu:eq :quote type)
                                           (prelude:%qq-compile expr)
                                           ())))
                               (mu:car expr))))
         ,(mu:cons :quote   (:lambda (expr) qq-expr))))))

(mu:intern :prelude "%qq-reader"
   (:lambda (char stream)
     (prelude:warn (prelude:%qq-compile (prelude:%qq-read stream)) "%qq-reader")))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
(mu:intern :prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (mu:boundp 'prelude:%debug-symbols%)
           ()
           (mu:intern :prelude "%debug-symbols%" (mu:ns-syms :list :prelude)))
      (mu:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (mu:eq (mu:sy-val symbol) value)
                        symbol
                        (mu:cdr list)))
                (mu:car list))
               list))
       prelude:%debug-symbols%)))

(mu:intern :prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))

;;;
;;; break loop
;;;
(mu:intern :prelude "break"
   (:lambda (except)
     (prelude:format :t ";;; entering break loop with exception~%" ())
     (prelude:%exceptf :t ";;; ~A on ~A by ~S, ~A~%" () except)
     (prelude:format :t ";;; :h for commands~%" ())
     (mu:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (mu:eq form prelude:%eof%)
                loop
                (:if (mu:eq form :h)
                     ((:lambda ()
                        (prelude:format :t "break help:~%" ())
                        (prelude:format :t ":d - describe exception value~%" ())
                        (prelude:format :t ":e - print the full exception~%" ())
                        (prelude:format :t ":r - return from break~%" ())
                        (prelude:format :t ":x - exit process~%" ())
                        (prelude:null loop)))
                     (:if (mu:eq form :d)
                          ((:lambda (value)
                             (prelude:describe (mu:cdr value) :t)
                             (prelude:null loop))
                           (prelude:%except-prop :value except))
                          (:if (mu:eq form :x)
                               (mu:exit 0)
                               (:if (mu:eq form :e)
                                    ((:lambda ()
                                       (prelude:%exceptf mu:std-out "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (prelude:null loop)))
                                    (:if (mu:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (prelude:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (prelude:null loop))))))))))
         ((:lambda ()
            (prelude:format :t "prelude:debug> " ())
            (mu:flush mu:std-out)
            (prelude:read :t () prelude:%eof%)))))
     ())))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude compiler
;;;

;;;
;;; special forms
;;;

;;;
;;; compile a lambda form to a function prelude type
;;;
;;; returns the compiled function
;;;
(mu:intern :prelude "%compile-lambda"
   (:lambda (form env)
      (:if (mu:eq (mu:length form) 1)
           (prelude:raise form 'prelude:%compile-lambda "malformed lambda expression")
           ((:lambda (lambda body)
               ((:lambda (lambda-desc)
                   ((:lambda (compiled-function)
                       (:if (prelude:%orf
                             (prelude:%lambda-prop :lexenv lambda-desc)
                             (prelude:%lambda-prop :rest lambda-desc))
                            (prelude:%make-funct lambda-desc compiled-function ())
                            compiled-function))
                    (mu:compile
                     `(:lambda
                       ,(prelude:%lambda-prop :bound lambda-desc)
                       ,@(prelude:%compile-lambda-body
                          lambda-desc
                          body
                          env)))))
                (prelude:%compile-lambda-list lambda env)))
            (mu:nth 1 form)
            (mu:nthcdr 2 form)))))

;;;
;;; compile a defmacro form to a bound macro symbol
;;;
;;; returns the macro symbol
;;;
(mu:intern :prelude "%compile-defmacro"
   (:lambda (form env)
      ((:lambda (macro-symbol lambda-form)
          (:if (prelude:symbolp macro-symbol)
               (prelude:%defmacro macro-symbol (prelude:%compile-lambda lambda-form env))
               (prelude:raise macro-symbol 'prelude:%compile-defmacro "not a symbol"))
          `(:quote ,macro-symbol))
       (mu:nth 1 form)
       (mu:nthcdr 1 form))))

;;;
;;; this if form can take two or three arguments
;;;
;;; returns an application of the if form
;;;
(mu:intern :prelude "%make-thunk"
  (:lambda (form env)
    (prelude:%compile
     `(:lambda () ,form)
     env)))

(mu:intern :prelude "%compile-if"
   (:lambda (form env)
      ((:lambda (length)
          (:if (prelude:%orf (mu:eq length 3) (mu:eq length 4))
               ((:lambda (t-arm f-arm)
                  (mu:compile `(:if ,(mu:nth 1 form) ,t-arm ,f-arm)))
                (:if (prelude:constantp (mu:nth 2 form))
                     (mu:nth 2 form)
                     `(prelude:apply ,(prelude:%make-thunk (mu:nth 2 form) env) ()))
                (:if (prelude:constantp (mu:nth 3 form))
                     (mu:nth 3 form)
                     `(prelude:apply ,(prelude:%make-thunk (mu:nth 3 form) env) ())))
               (prelude:raise form 'prelude:%compile-if "takes two or three arguments")))
       (mu:length form))))

;;;
;;; prelude compiler
;;;
;;; rewrites a function call/special call/macro call against an environment
;;; into something the runtime can execute. constants compile to themselves.
;;;
;;; returns a mu form or funct type (in the case of a define-prelude-lambda)
;;;
(mu:intern :prelude "%compile"
   (:lambda (form env)
      (:if (prelude:consp form)
           ((:lambda (function-form arg-list)
               (:if (prelude:keywordp function-form)
                    (mu:compile form)                    ; mu special form (:key ...)
                    (:if (mu:eq :symbol (mu:type-of function-form)) ; special form or macro
                         ((:lambda (special)
                             (:if special
                                  (mu:apply (mu:sy-val special) `(,form ,env))
                                  ((:lambda (macro-function)
                                      (:if macro-function
                                           (prelude:%compile-macro-call function-form arg-list env)
                                           (prelude:%compile-funcall function-form arg-list env)))
                                   (prelude:macro-function function-form env))))
                          (mu:cdr (prelude:%assoc
                                   function-form
                                   '((define-macro . prelude:%compile-defmacro)
                                     (if           . prelude:%compile-if)
                                     (lambda       . prelude:%compile-lambda)))))
                    (prelude:%compile-funcall function-form arg-list env))))
            (mu:car form)
            (mu:cdr form))
           form)))

;;;
;;; the external compile interface, compiles in
;;; the null environment
;;;
(mu:intern :prelude "compile"
   (:lambda (form)
     (prelude:%compile form ())))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; prelude types
;;;
(mu:intern :prelude "%types-ns" (mu:make-ns :%types))

(mu:intern :prelude "%prelude-type-p"
   (:lambda (value)
      (:if (prelude:structp value)
           (mu:eq :<type> (mu:st-type value))
           ())))

;;;
;;; type struct: #s(:<type> #(:t name property-alist))
;;;
;;; where the property-alist is ((:predicate . :predicate) ...)
;;;
(mu:intern :prelude "define-type"
   (:lambda (name properties)
      ((:lambda (str)
          (:if (prelude:listp properties)
               ((:lambda (symbol)
                   (:if (prelude:null symbol)
                        (mu:intern :%types str (mu:struct :<type> `(,str ,properties)))
                        (prelude:raise symbol 'prelude:define-type "type symbol bound")))
                (mu:ns-find :%types str))
               (prelude:raise properties 'prelude:define-type "not a property list")))
       (prelude:string name))))

(mu:intern :prelude "%type-predicate-map%"
   `(,(mu:cons :char (:lambda (t) (prelude:charp t)))
      ,(mu:cons :cons (:lambda (t) (prelude:consp t)))
      ,(mu:cons :fixnum (:lambda (t) (prelude:fixnump t)))
      ,(mu:cons :float (:lambda (t) (prelude:floatp t)))
      ,(mu:cons :func (:lambda (t) (prelude:functionp t)))
      ,(mu:cons :listp (:lambda (t) (prelude:listp t)))
      ,(mu:cons :map (:lambda (t) (mu:eq :map (mu:type-of t))))
      ,(mu:cons :keyword (:lambda (t) (prelude:keywordp t)))
      ,(mu:cons :namespc (:lambda (t) (prelude:namespacep t)))
      ,(mu:cons :string (:lambda (t) (prelude:stringp t)))
      ,(mu:cons :struct (:lambda (t) (prelude:structp t)))
      ,(mu:cons :symbol (:lambda (t) (mu:eq :symbol (mu:type-of t))))
      ,(mu:cons :symbolp (:lambda (t) (prelude:symbolp t)))
      ,(mu:cons :t (:lambda (t) :t))
      ,(mu:cons :vector (:lambda (t) (prelude:vectorp t)))))

(mu:intern :prelude "%type-predicate"
   (:lambda (type value)
      ((:lambda (predicate)
          (mu:apply predicate `(,value)))
       (mu:cdr (prelude:%assoc type prelude:%type-predicate-map%)))))

(mu:intern :prelude "%make-type"
    (:lambda (name property-values)
       ((:lambda (type-symbol)
           (prelude:%mapc
            (:lambda (property-value)
               (prelude:findl
                (:lambda (property-def)
                   (:if (mu:eq (mu:car property-value) (mu:car property-def))
                        ((:lambda (type value)
                            (:if (prelude:%type-predicate type value)
                                 property-def
                                 (prelude:raise property-value '%make-type "type mismatch")))
                         (mu:cdr property-def)
                         (mu:cdr property-value))
                        ()))
                (mu:sv-ref (mu:st-vec (mu:sy-val type-symbol)) 1)))
            property-values))
        (mu:ns-find prelude:%types-ns name))
       (mu:struct :<type> `(,name ,property-values))))

;;;
;;; returns () if not found
;;;
(mu:intern :prelude "%type-ref"
   (:lambda (prop-name struct)
      (:if (prelude:%prelude-type-p struct)
           (:if (prelude:keywordp prop-name)
                (prelude:%assoc prop-name (mu:sv-ref (mu:st-vec struct) 1))
                (prelude:raise struct 'prelude:%type-ref "not a prelude type"))
           (prelude:raise prop-name 'prelude:%type-ref "not a property name"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; describe
;;;
(mu:intern :prelude "%describe-function"
   (:lambda (fn stream)
      ((:lambda (view)
          (prelude:format stream
            "function: (~A bytes) :func ~A~%    name:  [~A]~%    arity: ~A~%    body:  ~A~%"
            `(,(mu:hp-size fn)
              ,(prelude:type-of fn)
              ,(mu:sv-ref view 2)
              ,(mu:sv-ref view 0)
              ,(mu:sv-ref view 1))))
       (mu:view fn))))

(mu:intern :prelude "%describe-prelude-type"
   (:lambda (value stream)
      (prelude:format stream
       "prelude-type: (~A bytes) :<type> ~A~%    type:  ~A~%    props: ~A~%"
       `(,(mu:hp-size value)
         ,(prelude:type-of value)
         ,(mu:sv-ref (mu:st-vec value) 0)
         ,(mu:sv-ref (mu:st-vec value) 0)))))

(mu:intern :prelude "%describe-struct"
   (:lambda (struct stream)
      (prelude:format stream
       "struct: (~A bytes) :struct ~A~%    type:  ~A~%    props: ~A~%"
       `(,(mu:hp-size struct)
         ,(prelude:type-of struct)
         ,(mu:st-type struct)
         ,(mu:st-vec struct)))))

(mu:intern :prelude "%describe-symbol"
   (:lambda (symbol stream)
      (:if (prelude:null symbol)
           (prelude:format stream
            "symbol: (~A bytes) :null null~%    ns:    ()~%    name:  :nil~%    value: ()~%"
            `(,(mu:hp-size symbol)))
           (:if (prelude:keywordp symbol)
                (prelude:format stream
                 "symbol: (~A bytes) :symbol keyword~%    ns:    ()~%    name:  ~A~%    value: ~A~%"
                 `(,(mu:hp-size symbol) ,(mu:sy-name symbol) ,(mu:sy-val symbol)))
                (:if (mu:boundp symbol)
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:    ~A~%    name:  ~A~%    value: ~A~%"
                          `(,(mu:hp-size symbol)
                            ,(mu:sv-ref view 0)
                            ,(mu:sv-ref view 1)
                            ,(mu:sv-ref view 2))))
                      (mu:view symbol))
                     ((:lambda (view)
                         (prelude:format stream
                          "symbol: (~A bytes) :symbol symbol~%    ns:      ~A~%    name:    ~A~%    unbound: :t~%"
                          `(,(mu:hp-size symbol)
                            ,(mu:sv-ref view 0)
                            ,(mu:sv-ref view 1)
                            ,(mu:sv-ref view 2))))
                      (mu:view symbol)))))))

(mu:intern :prelude "%describe-fixnum"
   (:lambda (fx stream)
      (prelude:format stream "fixnum: (~A bytes) :fixnum ~A~%    format: 61b signed integer~%    value:  ~A~%"
        `(,(mu:hp-size fx) ,(prelude:type-of fx) ,fx))))

(mu:intern :prelude "%describe-char"
   (:lambda (ch stream)
      (prelude:format stream "char: (~A bytes) :char ~A~%    format: 8b ASCII character~%    value:  ~A~%"
        `(,(mu:hp-size ch) ,(prelude:type-of ch) ,ch))))

(mu:intern :prelude "%describe-float"
   (:lambda (fl stream)
      (prelude:format stream
       "float: :float ~A~%    format: 32b IEEE single float~%    value:  ~A~%"
       `(,(prelude:type-of fl) ,fl))))

(mu:intern :prelude "%describe-string"
   (:lambda (str stream)
      (prelude:format stream
       "string: (~A bytes) :vector string~%    format: unsigned 8b character vector~%    length: ~A~%    value:  ~A~%"
       `(,(mu:hp-size str) ,(mu:sv-len str) ,str))))

(mu:intern :prelude "%describe-vector"
   (:lambda (vec stream)
      (prelude:format stream
       "vector: (~A bytes) :vector ~A~%    format: ~A~%    length: ~A~%    elements:  ~A~%"
       `(,(mu:hp-size vec) ,(prelude:type-of vec) ,(mu:sv-type vec) ,(mu:sv-type vec) ,vec))))

(mu:intern :prelude "%describe-cons"
   (:lambda (cons stream)
      (:if (prelude:dotted-pair-p cons)
           (prelude:format stream
            "dotted pair: (~A bytes) :cons ~A~%    value:  ~A~%"
            `(,(mu:hp-size cons) ,(prelude:type-of cons) ,cons))
           (prelude:format stream
            "cons: (~A bytes) :cons ~A~%    length: ~A~%    value:  ~A~%"
            `(,(mu:hp-size cons) ,(prelude:type-of cons) ,(mu:length cons) ,cons)))))

(mu:intern :prelude "describe"
   (:lambda (value stream)
      (mu:fix
       (:lambda (list)
          (:if (prelude:null list)
               ()
               ((:lambda (predicate fn)
                   (:if (mu:apply predicate `(,value))
                        ((:lambda ()
                            (mu:apply fn `(,value ,stream))
                            ()))
                        (mu:cdr list)))
               (mu:car (mu:car list))
               (mu:cdr (mu:car list)))))
       `(,(mu:cons prelude:functionp prelude:%describe-function)
         ,(mu:cons prelude:charp prelude:%describe-char)
         ,(mu:cons prelude:%prelude-type-p prelude:%describe-prelude-type)
         ,(mu:cons prelude:structp prelude:%describe-struct)
         ,(mu:cons prelude:symbolp prelude:%describe-symbol)
         ,(mu:cons prelude:fixnump prelude:%describe-fixnum)
         ,(mu:cons prelude:floatp prelude:%describe-float)
         ,(mu:cons prelude:stringp prelude:%describe-string)
         ,(mu:cons prelude:vectorp prelude:%describe-vector)
          ,(mu:cons prelude:consp prelude:%describe-cons)))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  environment
;;;
(mu:intern :prelude "environmentp"
   (:lambda (env)
      (:if (prelude:%prelude-type-p env)
           (mu:eq 'env (prelude:type-of env))
           ())))

(mu:intern :prelude "make-environment"
   (:lambda (bindings)
     (:if (prelude:listp bindings)
          ((:lambda (env)
             (prelude:warn env "make-environment: env")
             (prelude:mapc
              (:lambda (pair)
                ((:lambda (symbol value)
                   (:if (prelude:symbolp symbol)
                        (prelude:%add-env env symbol value)
                        (prelude:raise symbol 'prelude:make-environment "not a symbol")))
                 (mu:car pair)
                 (mu:cdr pair)))
              bindings)
             env)
           (prelude:%make-env))
          (prelude:raise symbol 'prelude:make-environment "not a symbol"))))

;;;
;;; implementation
;;;
(mu:intern :prelude "%get-env"
   (:lambda (env symbol)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%get-env "symbol not in map")
                          sym))
                  (mu:ns-find ns (mu:sy-name symbol))))
                 (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%get-env "not an env"))
         (prelude:raise symbol 'prelude:%get-env "not a symbol"))))

(mu:intern :prelude "%add-env"
   (:lambda (env symbol value)
     (:if (prelude:symbolp symbol)
         (:if (prelude:environmentp env)
              ((:lambda (ns)
                 ((:lambda (sym)
                     (:if (prelude:null sym)
                          (prelude:raise symbol 'prelude:%add-env "symbol already in map")
                          (mu:intern ns (mu:sy-name symbol) value)))
                  (mu:ns-find ns (mu:sy-name symbol))))
               (prelude:%env-prop :symbols env))
              (prelude:raise env 'prelude:%add-env "not an env"))
         (prelude:raise symbol 'prelude:%add-env "not a symbol"))))

(mu:intern :prelude "%make-env"
   (:lambda ()
     (prelude:%make-type "env" `(,(mu:cons :symbols (mu:make-ns))))))

(mu:intern :prelude "%env-prop"
   (:lambda (prop env)
      (:if (prelude:findl (:lambda (env-prop) (mu:eq prop env-prop)) '(:symbols))
           (mu:cdr (prelude:%type-ref prop env))
           (prelude:raise prop 'prelude:%env-prop "not an environment property"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;
(mu:untern :prelude "%debug-symbols%")

;;;
;;; [*exception descriptor*] #s(:except cond value source reason env)
;;;
;;; cond:   condition type symbol
;;; value:  exception raised on value
;;; source: function designator, usually a symbol
;;; reason: extended condition, usually a string
;;; env:    dynamic environment, list of frames
;;;
(mu:intern :prelude "exceptp"
   (:lambda (ex)
      (mu:eq (prelude:type-of ex) 'except)))

(mu:intern :prelude "%make-except"
   (:lambda (cond value source reason env)
      (prelude:%make-type "except"
        `(,(mu:cons :cond   cond)
          ,(mu:cons :value  value)
          ,(mu:cons :source source)
          ,(mu:cons :reason reason)
          ,(mu:cons :env    env)))))

(mu:intern :prelude "%except-prop"
   (:lambda (prop except)
      (:if (prelude:findl (:lambda (lambda-prop) (mu:eq prop lambda-prop)) '(:cond :value :source :reason :env))
           (prelude:%type-ref prop except)
           (prelude:raise prop 'prelude:%except-prop "not an except property"))))

(mu:intern :prelude "%exceptf"
   (:lambda (stream format-string printenv except)
     ((:lambda (cond value source reason env)
         (:if (prelude:null format-string)
              (prelude:format stream "~S ~S ~S ~S~%" `(,cond ,value ,source ,reason))
              (prelude:format stream format-string `(,cond ,value ,source ,reason)))
         (:if (prelude:%andf printenv env)
              ((:lambda ()
                  (prelude:format stream "env:~%" ())
                  (prelude:%mapc
                   (:lambda (frame)
                      (prelude:format stream "  ~A~T(~A ~A)~%"
                                   `(,(prelude:%format-mapped-symbol (mu:car frame))
                                     ,(mu:car frame)
                                     ,(mu:cdr frame))))
                   (mu:cdr env))))
              ()))
      (prelude:%except-prop :cond except)
      (prelude:%except-prop :value except)
      (prelude:%except-prop :source except)
      (prelude:%except-prop :reason except)
      (prelude:%except-prop :env except))))

(mu:intern :prelude "with-exception"
  (:lambda (handler thunk)
     (mu:with-ex
      (:lambda (value condition source)
         (:if (mu:eq condition :except)
              (prelude:apply handler `(,value))
              (prelude:apply handler `(,(prelude:%make-except
                                      condition
                                      value
                                      source
                                      "mu:raise"
                                      (mu:frames))))))
      (:lambda () (prelude:apply thunk ())))))

;;;
;;; exception flavors
;;;
(mu:intern :prelude "make-exception" prelude:%make-except)

(mu:intern :prelude "error"
   (:lambda (value format-string arg-list)
      (prelude:format mu:err-out format-string arg-list)
      (mu:raise
       (prelude:%make-except :error value :error "" ())
       :except)))

(mu:intern :prelude "raise"
   (:lambda (value source reason verbose)
      (:if verbose          
           (mu:raise
            (prelude:%make-except :error value source reason (mu:frames))
            :except)
           (mu:raise
            (prelude:%make-except :error value source reason ())
            :except))))

(mu:intern :prelude "debug"
   (:lambda (value message)
     (mu:write ";;; debug: " () mu:err-out)          
     (mu:write value () mu:err-out)
     (mu:write " " () mu:err-out)
     (mu:write message () mu:err-out)
     (mu:wr-char #\linefeed mu:err-out)
     value))

(mu:intern :prelude "warn"
   (:lambda (value message)
     (prelude:format mu:err-out ";;; warning: ~S ~A~%" `(,value ,message))
    value))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;
(mu:intern :prelude "1+"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (mu:fx-add n 1)
          (prelude:raise n 'prelude:1+ "not a fixnum"))))

(mu:intern :prelude "1-"
   (:lambda (n)
     (:if (prelude:fixnump n)
          (mu:fx-sub n 1)
          (prelude:raise n 'prelude:1- "not a fixnum"))))

(mu:intern :prelude "truncate"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(mu:fx-div n m) ,@(mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))
               (prelude:raise m 'prelude:truncate "not a fixnum"))
          (prelude:raise n 'prelude:truncate "not a fixnum"))))

;;; round to negative infinity
(mu:intern :prelude "floor"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(mu:fx-div n m) ,@(mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:floor "not a fixnum"))))

;;; round to positive infinity
(mu:intern :prelude "ceiling"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               `(,(mu:fx-div n m) ,@(mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))
               (prelude:raise m 'prelude:ceiling "not a fixnum"))
          (prelude:raise n 'prelude:ceiling "not a fixnum"))))

(mu:intern :prelude "mod"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (mu:cdr (prelude:floor m n))
               (prelude:raise m 'prelude:mod "not a fixnum"))
          (prelude:raise n 'prelude:mod "not a fixnum"))))

(mu:intern :prelude "rem"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (mu:cdr (prelude:truncate m n))
               (prelude:raise m 'prelude:rem "not a fixnum"))
          (prelude:raise n 'prelude:rem "not a fixnum"))))

;;; round to nearest
(mu:intern :prelude "round"
   (:lambda (n m)
     (:if (prelude:fixnump n)
          (:if (prelude:fixnump m)
               (mu:fx-sub (mu:fx-div n m) (mu:fx-sub n (mu:fx-mul m (mu:fx-div n m))))
               (prelude:raise m 'prelude:round "not a fixnum"))
          (prelude:raise n 'prelude:round "not a fixnum"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(mu:intern :prelude "%fmt-tilde"
  (:lambda (dest arg-list)
    (prelude:write-char #\~ dest)
    arg-list))

(mu:intern :prelude "%fmt-tab"
  (:lambda (dest arg-list)
    (prelude:write-char #\tab dest)
    arg-list))

(mu:intern :prelude "%fmt-percent"
  (:lambda (dest arg-list)
    (prelude:write-char #\linefeed dest)
    arg-list))

(mu:intern :prelude "%fmt-bang"
  (:lambda (dest arg-list)
    (mu:cdr arg-list)))

(mu:intern :prelude "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (prelude:fixnump (mu:car arg-list))
          ((:lambda ()
              (prelude:write (mu:car arg-list) () dest)
              (mu:cdr arg-list)))
          (prelude:raise (mu:car arg-list) 'prelude:format "argument to ~D not a fixnum"))))

(mu:intern :prelude "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (prelude:fixnump fx)
            (:if (prelude:zerop fx)
                 (prelude:write "0" () dest)
                 ((:lambda (str-stream)
                     (mu:fix
                      (:lambda (n)
                         (:if (prelude:%orf (prelude:null n) (mu:eq 0 n))
                              ()
                              ((:lambda ()
                                  (mu:write (prelude:schar "0123456789abcdef" (mu:logand n #xf)) () str-stream)
                                  (mu:fx-div n 16)))))
                      (:if (mu:fx-lt fx 0) (mu:fx-mul -1 fx) fx))
                     (:if (mu:fx-lt fx 0)
                          (mu:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (mu:fix
                          (:lambda (len)
                             (:if (prelude:zerop len)
                          0
                          ((:lambda ()
                              (mu:write (prelude:schar hexstr (mu:fx-sub len 1)) () dest)
                              (mu:fx-sub len 1)))))
                   (mu:sv-len hexstr)))
                  (mu:get-str str-stream)))
             (mu:open :string :output "")))
            (prelude:raise fx 'prelude:format "argument to ~X not a fixnum")))
      (mu:car arg-list))))

(mu:intern :prelude "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (prelude:write (mu:car arg-list) () dest)
    (mu:cdr arg-list)))

(mu:intern :prelude "%fmt-standard"
  (:lambda (dest arg-list)
    (prelude:write (mu:car arg-list) :t dest)
    (mu:cdr arg-list)))

;;;
;;; control ops
;;;
(mu:intern :prelude "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (prelude:apply (mu:sy-val fn) `(,dest ,arg-list))
            (prelude:raise dir 'prelude:format "unrecognized format directive")))
     (mu:cdr
      (prelude:%assoc
       dir
       '((#\A . prelude:%fmt-aesthetic)
         (#\S . prelude:%fmt-standard)
         (#\X . prelude:%fmt-hex)
         (#\D . prelude:%fmt-decimal)
         (#\T . prelude:%fmt-tab)
         (#\~ . prelude:%fmt-tilde)
         (#\% . prelude:%fmt-percent)
         (#\! . prelude:%fmt-bang)))))))

(mu:intern :prelude "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (mu:sv-len fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       `(,(mu:fx-add 2 nth) ,@(prelude:%fmt-op (prelude:schar fmt (mu:fx-add 1 nth)) dest argl))
                       ((:lambda ()
                           (prelude:write-char ch dest)
                           `(,(mu:fx-add 1 nth) ,@argl)))))
                (prelude:schar fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     `(0 ,@arg-list))
    (:if (prelude:null stream)
         (prelude:get-output-stream-string dest)
         ())))

(mu:intern :prelude "format"
   (:lambda (stream format-string arg-list)
      (:if (prelude:stringp format-string)
           (:if (prelude:listp arg-list)
                (:if (prelude:zerop (mu:sv-len format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (prelude:%fmt-loop stream dest format-string arg-list))
                      (:if (prelude:null stream)
                           (mu:open :string :output "")
                           (:if (mu:eq stream :t)
                                mu:std-out
                                (:if (prelude:streamp stream)
                                     stream
                                     (prelude:raise stream 'prelude:format "not a stream designator"))))))
                (prelude:raise arg-list 'prelude:format "not a list"))
           (prelude:raise fmt 'prelude:format "not a format string"))))
;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; compile applications
;;;

;;;
;;; argument lists
;;;
(mu:intern :prelude "%arg-list"
    (:lambda (arg-list)
      (prelude:foldr
       (:lambda (elt acc)
         `(mu:cons ,elt ,acc))
       ()
       arg-list)))

(mu:intern :prelude "%lambda-arg-list"
   (:lambda (function arg-list)
      (:if (prelude:%prelude-function-p function)
           ((:lambda (rest nreqs)
               (:if (prelude:%andf rest (prelude:zerop nreqs))
                    `(mu:cons ,(prelude:%arg-list arg-list) ())
                    ((:lambda (reqs rest)
                        (prelude:%arg-list `(,@reqs ,(prelude:%arg-list rest))))
                     (prelude:dropr arg-list (mu:fx-sub (mu:length arg-list) nreqs))
                     (prelude:dropl arg-list nreqs))))
            (prelude:%funct-prop :rest function)
            (prelude:%funct-prop :arity function))
           (prelude:%arg-list arg-list))))

(mu:intern :prelude "%quoted-lambda-arg-list"
  (:lambda (fn args)
    (:if (prelude:%prelude-function-p fn)
         (:if (prelude:%funct-prop :rest fn)
              (:if (prelude:%andf (prelude:%funct-prop :rest fn) (prelude:zerop (prelude:%funct-prop :arity fn)))
                   `(,args)
                   ((:lambda (reqs rest)
                      (prelude:append
                       reqs
                       `(,rest)))
                    (prelude:dropr args (mu:fx-sub (mu:length args) (prelude:%funct-prop :arity fn)))
                    (prelude:dropl args (prelude:%funct-prop :arity fn))))
              args)
         args)))

;;;
;;; compiled argument lists
;;;
(mu:intern :prelude "%compile-arg-list"
   (:lambda (arg-list env)
      (prelude:foldr
       (:lambda (elt acc)
          `(mu:cons ,@(mu:cons elt `(,acc))))
       ()
       (prelude:mapcar
        (:lambda (form)
           (prelude:%compile form env))
        arg-list))))

(mu:intern :prelude "%compile-lambda-arg-list"
   (:lambda (function arg-list env)
     (prelude:%lambda-arg-list function (prelude:%mapcar (:lambda (elt) (prelude:%compile elt env)) arg-list))))

(mu:intern :prelude "%compile-quoted-lambda-arg-list"
   (:lambda (function arg-list env)
     (prelude:%compile-lambda-arg-list function (prelude:%mapcar (:lambda (elt) (prelude:%compile elt env)) arg-list))))

;;;
;;; compile-funcall
;;;
;;; compile function applications to mu-appliable forms
;;;
(mu:intern :prelude "%compile-lambda-call"
   (:lambda (lambda-form arg-list env)
      ((:lambda (compiled-function)
        (:if (prelude:functionp compiled-function)
             (:if (prelude:%prelude-function-p compiled-function)
                  `(prelude:%funct-apply-quoted ,compiled-function ,(prelude:%compile-lambda-arg-list compiled-function arg-list env))
                  `(mu:apply ,compiled-function ,(prelude:%compile-arg-list arg-list env)))
             (prelude:raise compiled-function 'prelude:%compile-lambda-call "illegal function call")))
     (prelude:%compile lambda-form env))))

(mu:intern :prelude "%compile-symbol-call"
   (:lambda (function-symbol arg-list env)
      (:if (prelude:boundp function-symbol)
           ((:lambda (function)
               (:if (prelude:functionp function)
                    (:if (prelude:%prelude-function-p function)
                         `(prelude:%funct-apply-quoted ,function ,(prelude:%compile-lambda-arg-list function arg-list env))
                         `(mu:apply ,function ,(prelude:%compile-arg-list arg-list env)))
                    (prelude:raise function 'prelude:%compile-symbol-call "illegal function call")))
            (prelude:symbol-value function-symbol))
           `(prelude:apply ,function-symbol ,(prelude:%compile-arg-list arg-list env)))))

(mu:intern :prelude "%compile-funcall"
   (:lambda (function-form arg-list env)
      (:if (prelude:consp function-form)
           (prelude:%compile-lambda-call function-form arg-list env)
           (:if (mu:eq :symbol (mu:type-of function-form))
                (prelude:%compile-symbol-call function-form arg-list env)
                (prelude:raise function-form 'prelude:%compile-funcall "illegal function call")))))

;;;
;;; apply closure to argument list
;;;
(mu:intern :prelude "%funct-apply"
   (:lambda (funct arg-list)
      ((:lambda (env)
        (prelude:%mapc mu:fr-push env)
        ((:lambda (value)
           (prelude:%mapc (:lambda (frame) (mu:fr-pop (mu:car frame))) env)
           value)
         ((:lambda (mu-fn)
             (mu:apply mu-fn (mu:eval (prelude:%lambda-arg-list funct arg-list))))
          (prelude:%funct-prop :fn funct))))
        (prelude:%funct-prop :env funct))))

(mu:intern :prelude "%funct-apply-quoted"
   (:lambda (funct arg-list)
      ((:lambda (env)
        (prelude:%mapc mu:fr-push env)
        ((:lambda (value)
           (prelude:%mapc (:lambda (frame) (mu:fr-pop (mu:car frame))) env)
           value)
         ((:lambda (mu-fn)
             (mu:apply mu-fn arg-list))
          (prelude:%funct-prop :fn funct))))
        (prelude:%funct-prop :env funct))))
;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; closures
;;;
;;;
;;; predicates
;;;
(mu:intern :prelude "%prelude-function-p"
   (:lambda (fn)
      (mu:eq 'funct (prelude:type-of fn))))

(mu:intern :prelude "functionp"
   (:lambda (function)
      (:if (mu:eq :func (mu:type-of function))
           :t
           (mu:eq 'funct (prelude:type-of function)))))

;;;
;;; [*function descriptor*] #s(:<type> :funct (lambda narg-list fn env))
;;;
;;; lambda: lambda type
;;; nargs:  number of reqs plus rest
;;; fn:     implementation
;;; env:    dynamic environment, list of frames
;;;
;;; function
;;;
(mu:intern :prelude "%make-funct"
   (:lambda (lambda fn env)
      ((:lambda (nargs)
          (prelude:%make-type "funct"
             `(,(mu:cons :lambda lambda)
               ,(mu:cons :arity  nargs)
               ,(mu:cons :fn     fn)
               ,(mu:cons :env    env))))
       (mu:fx-sub
        (mu:length (prelude:%lambda-prop :bound lambda))
        (:if (prelude:%lambda-prop :rest lambda) 1 0)))))

(mu:intern :prelude "%funct-prop"
   (:lambda (prop funct)
      (:if (prelude:findl (:lambda (funct-prop) (mu:eq prop funct-prop)) '(:lambda :arity :fn :env))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'prelude:%function-prop "property not bound")
               (mu:cdr ref))
            (prelude:%type-ref prop funct))
           (prelude:%lambda-prop prop (prelude:%funct-prop :lambda funct)))))

;;;
;;; create closed environment
;;;
;;; retruns env
;;;
(mu:intern :prelude "%closure-env"
   (:lambda (lambda body env)
      ((:lambda (bound-symbols)
          (prelude:%foldl
           (:lambda (elt acc)
              (:if (prelude:consp elt)
                   ((:lambda (function-form)
                       (:if (prelude:consp function-form)
                            `(,@(prelude:%closure-env lambda function-form env) ,@acc)
                            `(,@(prelude:%closure-env lambda (mu:cdr elt) env) ,@acc)))
                    (mu:car elt))
                   (:if (mu:eq :symbol (mu:type-of elt))
                        (:if (prelude:uninternedp elt)
                             ((:lambda (symbol)
                                 (:if symbol
                                      acc
                                      ((:lambda (binding-lambda)
                                          (:if binding-lambda
                                                `(,(mu:cons form binding-lambda) ,@acc)
                                                acc))
                                       (prelude:findl
                                        (:lambda (lambda)
                                           (prelude:findl
                                            (:lambda (symbol) (mu:eq symbol form))
                                            (prelude:%type-ref :bound lambda)))
                                        env))))
                                 (prelude:findl (:lambda (symbol) (mu:eq symbol form)) bound-symbols))
                             acc)
                        acc)))
           ()
           body))
       (prelude:%type-ref :bound lambda))))

;;;
;;; %close-env = > closure
;;;
;;; mu:frames => list of [*frame descriptor*] (function . vector)
;;; env => list of (symbol . lambda)
;;;
(mu:intern :prelude "%close-env"
  (:lambda (closure env)
     ((:lambda (lambda fn env)
         (prelude:%make-funct lambda fn env))
      (prelude:%make-lambda
       (prelude:%lambda-prop :bound fn)
       (prelude:%lambda-prop :rest fn)
       (mu:frames))
      (prelude:%funct-prop :fn fn))))

;;;
;;; compile closed lambda => function
;;;
(mu:intern :prelude "%compile-function"
   (:lambda (form env)
      ((:lambda (lambda)
          ((:lambda (fn)
              (prelude:%make-funct lambda fn ()))
           (mu:compile
            `(:lambda
              ,(prelude:%lambda-prop :bound lambda)
              ,(prelude:%compile-lambda-body lambda (mu:nthcdr 2 form) env)))))
       (prelude:%compile-lambda-list (mu:nth 1 form) env))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; inspect
;;;
(mu:intern :prelude "%inspect-function"
   (:lambda (obj)
      ((:lambda (view)
          ((:lambda (tags)
             (mu:struct :inspect `(function ,(mu:hp-size obj) ,tags)))
           `(,(mu:cons :arity (mu:sv-ref view 0))
             ,(mu:cons :body (mu:sv-ref view 1)))))
       (mu:view obj))))

(mu:intern :prelude "%inspect-type"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)
            (mu:struct :inspect `(:<type> ,(mu:hp-size obj) ,tags)))
          `(,(mu:cons :type (mu:sv-ref (mu:st-vec obj) 0))
            ,(mu:cons :values (mu:sv-ref (mu:st-vec obj) 1)))))
      (mu:view obj))))

(mu:intern :prelude "%inspect-struct"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (mu:struct :inspect `(struct ,(mu:hp-size obj) ,tags)))
          `(,(mu:cons :type (mu:st-type obj))
            ,(mu:cons :values (mu:st-vec obj)))))
      (mu:view obj))))

(mu:intern :prelude "%inspect-symbol"
   (:lambda (obj)
     ((:lambda (view)
         ((:lambda (tags)  
            (mu:struct :inspect `(symbol ,(mu:hp-size obj) ,tags)))
          `(,(mu:cons :ns (mu:sv-ref view 0))
            ,(mu:cons :name (mu:sv-ref view 1))
            ,(mu:cons :value (mu:sv-ref view 2)))))
      (mu:view obj))))

(mu:intern :prelude "%inspect-const"
   (:lambda (obj)
     (mu:struct :inspect `(,(prelude:type-of obj) ,(mu:hp-size obj) ()))))

(mu:intern :prelude "%inspect-vector"
   (:lambda (obj)
     ((:lambda (tags)          
         (mu:struct :inspect `(vector ,(mu:hp-size obj) ,tags)))
      `(,(mu:cons :type (mu:sv-type obj))
        ,(mu:cons :length (mu:sv-len obj))))))

(mu:intern :prelude "%inspect-cons"
   (:lambda (obj)
     ((:lambda (tags)          
         (mu:struct :inspect `(cons ,(mu:hp-size obj) ,tags)))
      `(,(mu:cons :car (mu:car obj))
        ,(mu:cons :cdr (mu:cdr obj))))))

(mu:intern :prelude "%inspect"
   (:lambda (value)
      (mu:fix
       (:lambda (loop)
          (:if (prelude:structp loop)
               loop
               (:if (prelude:null loop)
                    (mu:struct :inspect `(unknown 0 ()))
                    ((:lambda (predicate fn)
                       (:if (mu:apply predicate `(,value))
                            (mu:apply fn `(,value))
                            (mu:cdr loop)))
                     (mu:car (mu:car loop))
                     (mu:cdr (mu:car loop))))))
       `(,(mu:cons prelude:%prelude-type-p prelude:%inspect-type)
         ,(mu:cons prelude:charp prelude:%inspect-const)
         ,(mu:cons prelude:consp prelude:%inspect-cons)
         ,(mu:cons prelude:fixnump prelude:%inspect-const)
         ,(mu:cons prelude:floatp prelude:%inspect-const)
         ,(mu:cons prelude:functionp prelude:%inspect-function)
         ,(mu:cons prelude:structp prelude:%inspect-struct)
         ,(mu:cons prelude:symbolp prelude:%inspect-symbol)
         ,(mu:cons prelude:vectorp prelude:%inspect-vector)))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lambda form
;;;
;;; [*lambda form*]
;;; (lambda ([symbol ...] [&rest rest-symbol]) . body)
;;;

;;;
;;; [*lambda descriptor*] #s(:<type> :lambda (bound rest lexenv))
;;;
;;; bound:  bound symbols
;;; rest:   rest symbol or nil
;;; lexenv: lexical environment, list of lambdas
;;;
(mu:intern :prelude "%lambdap"
   (:lambda (fn)
      (mu:eq (prelude:type-of fn) 'lambda)))

(mu:intern :prelude "%make-lambda"
   (:lambda (lambda-list rest-sym env)
      (prelude:%make-type "lambda"
         `(,(mu:cons :bound  lambda-list)
           ,(mu:cons :rest   rest-sym)
           ,(mu:cons :lexenv env)))))

(mu:intern :prelude "%lambda-prop"
   (:lambda (prop lambda)
      (:if (prelude:findl (:lambda (lambda-prop) (mu:eq prop lambda-prop)) '(:bound :rest :lexenv))
           ((:lambda (ref)
               ;;; (prelude:raise-when prelude:null prop 'lambda-prop "property not bound")
               (mu:cdr ref))
            (prelude:%type-ref prop lambda))
           (prelude:raise prop 'prelude:%lambda-prop "not a lambda property"))))

;;;
;;; `%compile-lambda-list` compiles a lambda form to a lambda descriptor
;;;
;;; `lambda:` whole lambda form, with leading symbol
;;; `env:`    lexical environment
;;;
;;; `returns:` lambda descriptor
;;;
(mu:intern :prelude "%compile-lambda-list"
   (:lambda (lambda env)
      (:if (prelude:findl (:lambda (el) (prelude:null (mu:eq :symbol (mu:type-of el)))) lambda)
           (prelude:raise lambda 'prelude:%compile-lambda-list "list syntax")
           ((:lambda (desc)
               (:if (prelude:fixnump desc)
                    (prelude:%make-lambda lambda () env)
                    desc))
            (prelude:%foldl
             (:lambda (el acc)
                (:if (prelude:%lambdap acc)
                     acc
                     (:if (mu:eq '&rest el)
                          (:if (mu:eq (mu:length lambda) (mu:fx-add 2 acc))
                               (prelude:%make-lambda
                                `(,@(prelude:dropr lambda 2) ,@(prelude:dropl lambda (mu:fx-sub (mu:length lambda) 1)))
                                (mu:nth (mu:fx-sub (mu:length lambda) 1) lambda)
                                env)
                               (prelude:raise lambda 'prelude:%compile-lambda-list "rest botch"))
                          (mu:fx-add 1 acc))))
             0
             lambda)))))

;;;
;;; `%compile-lambda-body` resolves body symbols with `prelude::compile-symbol`,
;;; uses the prelude compiler for everything else, and returns a compiled body.
;;;
;;; if a body returns a lambda expression, generate a closure body.
;;;
;;; `lambda-desc:` lambda descriptor
;;; `body:` list of body forms
;;; `env:` runtime lexical env
;;;
;;; `returns:` compiled body/closure
;;;
(mu:intern :prelude "%compile-lambda-body"
   (:lambda (lambda-desc body env)
      (:if (prelude:null body)
           ()
           ((:lambda (env)
               (prelude:maplist
                (:lambda (body-cdr)
                   (:if (mu:cdr body-cdr)
                        ((:lambda (value)
                            (prelude:%compile value env))
                            (mu:car body-cdr))
                        ((:lambda (form)
                            (:if (prelude:consp form)
                                 (:if (mu:eq 'lambda (mu:car form))
                                      (prelude:%compile-function form env)
                                      (prelude:%compile form env))
                                 (prelude:%compile form env)))
                         (mu:car body-cdr))))
                body))
          `(,lambda-desc ,@env)))))

#|
;;;
;;; resolving body symbols
;;;
;;; *%frame-ref* looks up *symbol* in the lexical environment.
;;;
;;; if found return a compiled frame reference, otherwise *nil*.
;;;
;;; `symbol:` symbol to be compiled
;;; `env:` lexical compiled environment
;;;
;;; `returns:` *frame-ref* or *nil*
;;;
(mu:intern :prelude "%frame-ref"
  (:lambda (symbol env)
    (prelude:%foldl
     (:lambda (frame acc)
       (:if acc
            acc
            ((:lambda (offset)
               (:if offset
                    (mu:compile
                     `(mu:fr-ref
                       ,(mu:cdr (prelude:%type-ref :func frame))
                       ,offset))
                    ()))
            (prelude:positionl
             (:lambda (el) (mu:eq el symbol))
             (mu:cdr (prelude:%type-ref :bound frame))))))
     ()
     env)))
|#
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; lists
;;;
(mu:intern :prelude "consp" (:lambda (t) (mu:eq :cons (mu:type-of t))))
(mu:intern :prelude "null" (:lambda (t) (mu:eq () t)))
(mu:intern :prelude "listp" (:lambda (t) (:if (prelude:consp t) :t (prelude:null t))))

(mu:intern :prelude "dotted-pair-p"
  (:lambda (list)
    (:if (prelude:consp list)
         (prelude:atom (mu:cdr list))
         ())))

;;;
;;; finds
;;;
(mu:intern :prelude "findl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (mu:cdr
                 (mu:fix
                  (:lambda (loop)
                     (:if (mu:eq :t (mu:car loop))
                          loop
                          (:if (mu:cdr loop)
                               ((:lambda (el)
                                   (:if (prelude:apply function `(,el))
                                        `(:t ,@el)
                                        `(() ,@(mu:nthcdr 2 loop))))
                                (mu:nth 1 loop))
                               loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:findl "not a list"))
           (prelude:raise function 'prelude:findl "not a function"))))

(mu:intern :prelude "findr"
   (:lambda (function list)
     (prelude:findl function (prelude:reverse list))))

;;;
;;; position
;;;
(mu:intern :prelude "positionl"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (:if list
                    ((:lambda (length)
                        (mu:car (mu:fix
                           (:lambda (loop)
                              (:if (prelude:listp loop)
                                   loop
                                   (:if (mu:fx-lt length loop)
                                        ()
                                        (:if (prelude:apply function `(,(mu:nth loop list)))
                                             `(,loop)
                                             (prelude:1+ loop)))))
                           0)))
                     (mu:length list))
                    ())
               (prelude:raise list 'prelude:positionl "not a list"))
          (prelude:raise function 'prelude:positionl "not a function"))))

(mu:intern :prelude "positionr"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (:if (prelude:null list)
                     ()
                     (mu:car
                      (mu:fix
                       (:lambda (loop)
                          (:if (prelude:listp loop)
                               loop
                               (:if (mu:fx-lt loop 0)
                                    ()
                                    (:if (prelude:apply function `(,(mu:nth loop list)))
                                         `(,loop)
                                         (prelude:1- loop)))))
                       (mu:fx-sub (mu:length list) 1))))
                (prelude:raise list 'prelude:positionr "not a list"))
           (prelude:raise function 'prelude:positionr "not a function"))))

;;;
;;; fixed argument list functions
;;;
(mu:intern :prelude "append"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldr
            (:lambda (el acc)
               (:if (prelude:listp el)
                    (prelude:foldr mu:cons acc el)
                    el))
            ()
            list)
      (prelude:raise list 'prelude:append "not a list"))))

(mu:intern :prelude "reverse"
   (:lambda (list)
      (:if (prelude:listp list)
           (prelude:foldl mu:cons () list)
           (prelude:raise list 'prelude:reverse "not a list"))))

(mu:intern :prelude "dropl"
   (:lambda (list n)
      (:if (prelude:listp list)
         (:if (prelude:fixnump n)
              (mu:nthcdr n list)
              (prelude:raise n 'prelude:dropl "not a fixnum"))
         (prelude:raise list 'prelude:dropl "not a list"))))

(mu:intern :prelude "dropr"
   (:lambda (list n)
      (:if (prelude:listp list)
           (:if (prelude:fixnump n)
                (prelude:foldl
                 mu:cons
                 ()
                 ((:lambda (length)
                     (prelude:foldl
                      (:lambda (el last)
                         (:if (mu:fx-lt (mu:length last) (mu:fx-sub length n))
                              `(,el ,@last)
                              last))
                      ()
                      list))
                  (mu:length list)))
                (prelude:raise n 'prelude:dropr "not a fixnum"))
           (prelude:raise list 'prelude:dropr "not a list"))))

(mu:intern :prelude "last"
   (:lambda (list)
      (:if (prelude:listp list)
           (mu:fix
            (:lambda (loop)
               (:if (prelude:null list)
                    loop
                    ((:lambda (cdr)
                        (:if (prelude:consp cdr)
                             cdr
                             loop))
                     (mu:cdr loop))))
            list)
           (prelude:raise list 'prelude:last "not a list"))))

;;;
;;; count
;;;
(mu:intern :prelude "count"
   (:lambda (item list)
     (:if (prelude:listp list)
          (prelude:foldl          
           (:lambda (elt acc)
              (:if (mu:eq elt item)
                   (prelude:1+ acc)
                   acc))
           0
           list)
          (prelude:raise list 'prelude:count "not a list"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader utilities
;;;

;;;
;;; loader
;;;
(mu:intern :prelude "load"
  (:lambda (path verbose)
    (:if (prelude:stringp path)
         ((:lambda (ifs)
            (:if (prelude:streamp ifs)
                 ((:lambda ()
                    (:if verbose
                         (prelude:format :t ";;; loading ~A~%" `(,path))
                         ())
                    ((:lambda (status)
                       (:if (mu:eq prelude:%eof% status)
                            :t
                            ()))
                     (mu:fix
                        (:lambda (loop)
                           (:if (mu:eq prelude:%eof% loop)
                                prelude:%eof%
                                (prelude:with-exception
                                    (:lambda (exception)
                                      (prelude:%exceptf mu:err-out "load exception: ~A~%" () exception))
                                    (:lambda ()
                                      ((:lambda (form)
                                          (:if (mu:eq prelude:%eof% form)
                                               prelude:%eof%
                                               ((:lambda (value)
                                                  (:if verbose
                                                       (prelude:format :t "~A~%" `(,value))
                                                       ())
                                                  (prelude:null loop))
                                                (mu:eval (mu:compile form)))))
                                       (mu:read ifs () prelude:%eof%))))))
                       ()))))
                  (prelude:raise prelude:streamp ifs 'prelude:load "cannot open input file")))
            (mu:open :file :input path))
          (prelude:raise path 'prelude:load "not a file designator"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; logging
;;;
(mu:intern :prelude "%logf"
   (:lambda (stream format-satring arg-list)
      (prelude:apply prelude:format `(,stream ,format-string ,arg-list))))

(mu:intern :prelude "logf"
   (:lambda (env format-string arg-list)
      ((:lambda (stream)
          (prelude:apply prelude:format `(,stream ,format-string ,arg-list))))
      (prelude:%env-prop :log env)))

(mu:intern :prelude "%open-log-file"
   (:lambda (file-name env)
      (mu:open :file :output file-name)))

;;;
;;; exception flavors
;;;
(mu:intern :prelude "log-error"
   (:lambda (format-string value env)
      (prelude:logf env format-string `(,value))
      (prelude:error format-string value)))

(mu:intern :prelude "log-raise"
   (:lambda (format-string value source reason env)
      (prelude:logf env format-string `(,value ,source ,reason ,env))
      (prelude:raise value source reason)))

(mu:intern :prelude "log-warn"
   (:lambda (value message env)
      (prelude:logf env ";;; warning: ~S ~A~%" `(,value ,message))
      value))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; macro definition, compilation, and expansion
;;;

;;;
;;; Macro definitions are compiled like lambdas (though eventually with a slighlty
;;; different lambda list including &whole).
;;;
;;; Macro calls are expanded at compile time.
;;;
;;; Common Lisp adds an environment argument here for lexical macro expanders.
;;;

;;;
;;; macro environment
;;;
(mu:intern :prelude "%map-macro-symbol-ns"
   (:lambda (symbol)
      ((:lambda (ns-name)
          (mu:ns-find prelude:%macro-ns% ns-name))
       (mu:sy-name (mu:sy-ns symbol)))))

(mu:intern :prelude "%make-macro-ns"
   (:lambda (ns)
      ((:lambda (macro-ns)
          (mu:intern prelude:%macro-ns% (mu:sy-name ns) macro-ns)
          macro-ns)
       (prelude:%gen-anon-ns))))

(mu:intern :prelude "%defmacro"
   (:lambda (symbol macro-function)
      ((:lambda (macro-ns symbol-ns)
         (:if macro-ns
              (mu:intern macro-ns symbol-ns macro-function)
              (mu:intern (prelude:%make-macro-ns symbol-ns) (mu:sy-name symbol) macro-function))
         symbol)
       (prelude:%map-macro-symbol-ns symbol)
       (mu:sy-ns symbol))))

(mu:intern :prelude "%find-macro"
   (:lambda (symbol env)
      ((:lambda (macro-ns symbol-ns symbol-name)
          (:if macro-ns
               (mu:ns-find (mu:sy-val macro-ns) symbol-name)
               ()))
       (mu:ns-find
        prelude:%macro-ns%
        (mu:sy-name (mu:sy-ns symbol)))
       (mu:sy-name (mu:sy-ns symbol))
       (mu:sy-name symbol))))

(mu:intern :prelude "%compile-macro-call"
   (:lambda (macro-symbol arg-list env)
       ((:lambda (macro-function)
          ((:lambda (expanded-form)
             (prelude:%compile expanded-form env))
           (:if (prelude:%prelude-function-p macro-function)
                (prelude:%funct-apply-quoted
                 macro-function
                 (mu:eval (prelude:%compile-lambda-arg-list macro-function arg-list env)))
                (mu:apply macro-function arg-list))))
      (prelude:macro-function macro-symbol env))))

;;;
;;; macroexpand-1 expands a macro call once.
;;;
(mu:intern :prelude "macroexpand-1"
   (:lambda (form env)
     (:if (prelude:consp form)
          ((:lambda (fn-symbol arg-list)
             (:if (mu:eq :symbol (mu:type-of fn-symbol))
                  ((:lambda (macro-function)
                     (:if macro-function
                          (:if (prelude:%prelude-function-p macro-function)
                               (prelude:%funct-apply-quoted macro-function (prelude:%quoted-lambda-arg-list macro-function arg-list))
                               (mu:apply macro-function arg-list))
                          form))
                   (prelude:macro-function fn-symbol env))
                  form))
           (mu:car form)
           (mu:cdr form))
          form)))

;;;
;;; macroexpand: expand a form until it is no longer a macro call
;;;
(mu:intern :prelude "macroexpand"
   (:lambda (form env)
     ((:lambda (expanded)
          (:if (mu:eq form expanded)
             expanded
             (prelude:macroexpand expanded env)))
       (prelude:macroexpand-1 form env))))

;;;
;;; macro-function: map a symbol to a macro function
;;;
;;; acts as a predicate
;;;
(mu:intern :prelude "macro-function"
   (:lambda (symbol env)
      (:if (mu:eq :symbol (mu:type-of symbol))
           ((:lambda (macro)
               (:if macro
                    (mu:sy-val macro)
                    ()))
            (prelude:%find-macro symbol env))
          ())))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; folds and maps
;;;
(mu:intern :prelude "%foldl"
  (:lambda (function init list)
     (mu:cdr
      (mu:fix
       (:lambda (arg)
          ((:lambda (list acc)
             (:if list
                  `(,(mu:cdr list)
                     ,@(mu:apply function `(,(mu:car list) ,acc)))
                   arg))
           (mu:car arg)
           (mu:cdr arg)))
       `(,list ,@init)))))

(mu:intern :prelude "foldl"
  (:lambda (function init list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (mu:cdr
                (mu:fix
                 (:lambda (arg)
                    ((:lambda (list acc)
                        (:if list
                             (mu:cons
                              (mu:cdr list)
                              (prelude:apply function `(,(mu:car list) ,acc)))
                             arg))
                     (mu:car arg)
                     (mu:cdr arg)))
                 `(,list ,@init)))
               (prelude:raise list 'prelude:foldl "not a list"))
          (prelude:raise function 'prelude:foldl "not a function"))))

(mu:intern :prelude "foldr"
   (:lambda (function init list)
    (:if (prelude:functionp function)
         (:if (prelude:listp list)
              (prelude:foldl function init (prelude:reverse list))
              (prelude:raise list 'prelude:foldr "not a list"))
         (prelude:raise function 'prelude:foldr "not a function"))))

;;;
;;; maps
;;;
(mu:intern :prelude "%mapc"
   (:lambda (function list)
     (mu:fix
      (:lambda (list-cdr)
        (:if list-cdr
             ((:lambda ()
                 (mu:apply function `(,(mu:car list-cdr)))
                 (mu:cdr list-cdr)))
             ()))
      list)
     list))

(mu:intern :prelude "mapc"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapc function list)
               (prelude:raise list 'prelude:mapc "not a list"))
          (prelude:raise function 'prelude:mapc "not a function"))
     list))

(mu:intern :prelude "%mapcar"
   (:lambda (function list)
      (prelude:%foldl
       (:lambda (elt acc)
          `(,@acc ,(mu:apply function `(,elt))))
       ()
       list)))

(mu:intern :prelude "mapcar"
   (:lambda (function list)
     (:if (prelude:functionp function)
          (:if (prelude:listp list)
               (prelude:%mapcar function list)
               (prelude:raise list 'prelude:mapcar "not a list"))
          (prelude:raise function 'prelude:mapcar "not a function"))))

(mu:intern :prelude "mapl"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (mu:fix
                 (:lambda (list)
                    (:if list
                         ((:lambda ()
                            (prelude:apply function `(,list))
                            (mu:cdr list)))
                         ()))
                 list)
                (prelude:raise list 'prelude:mapl "not a list"))
           (prelude:raise function 'prelude:mapl "not a function"))
      list))

(mu:intern :prelude "maplist"
   (:lambda (function list)
      (:if (prelude:functionp function)
           (:if (prelude:listp list)
                (mu:car
                 (mu:fix
                  (:lambda (loop)
                     ((:lambda (acc list)
                         (:if list
                              (mu:cons
                               `(,@acc ,(prelude:apply function `(,list)))
                                (mu:cdr list))
                              loop))
                      (mu:car loop)
                      (mu:cdr loop)))
                  `(() ,@list)))
                (prelude:raise list 'prelude:maplist "not a list"))
           (prelude:raise function 'prelude:maplist "not a function"))))

(mu:intern :prelude "%assoc"
   (:lambda (item alist)
      (mu:car
       (mu:fix
        (:lambda (lst)
           (:if lst
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq item (mu:car entry))
                              lst
                              cdr)
                         cdr))
                 (mu:car lst)
                 (mu:cdr lst))
                ()))
        alist))))

(mu:intern :prelude "assoc"
   (:lambda (item alist)
     (:if (prelude:listp alist)
          (prelude:%assoc item alist)
          (prelude:raise alist 'prelude:assoc "not an alist"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  prelude namespace
;;;

;;;
;;; namespace implementation
;;;
(mu:intern :prelude "namespacep"
   (:lambda (ns)
      (mu:eq (prelude:type-of ns) 'namespace)))

(mu:intern :prelude "%make-namespc"
   (:lambda (name uses)
     (prelude:%make-type "namespc"
       `(,(mu:cons :name name)
          ,(mu:cons :intern (mu:make-ns))
          ,(mu:cons :extern (mu:make-ns))
          ,(mu:cons :ch-mac (mu:make-ns))
          ,(mu:cons :sy-mac (mu:make-ns))
          ,(mu:cons :uses uses)))))

(mu:intern :prelude "%namespace-prop"
   (:lambda (prop namespace)
      (:if (prelude:findl
            (:lambda (lambda-prop)
               (mu:eq prop lambda-prop))
            '(:name :intern :extern :ch-mac :rd-mac :sy-mac :uses))
           (prelude:%type-ref prop namespace)
           (prelude:raise prop 'prelude:%namespace-prop "not a namespace property"))))

;;;
;;; namespace functions
;;;
(mu:intern :prelude "make-namespace"
   (:lambda (name uses)
      (:if (prelude:stringp name)
           (:if (prelude:listp uses)
                (prelude:%make-namespc name uses)
                (prelude:raise uses 'prelude:make-namespace "not a uses list"))
           (prelude:raise name 'prelude:make-namespace "not a name string"))))

(mu:intern :prelude "intern"
   (:lambda (symbol value)          
      (:if (prelude:uninternedp symbol)
           ((:lambda (sym)
               (:if sym
                    (:if (prelude:boundp sym)
                         symbol
                         (mu:intern (prelude:%reader-get-ns) (mu:sy-name symbol) value))
                    (mu:intern (prelude:%reader-get-ns) (mu:sy-name symbol) value)))
            (mu:ns-find (prelude:%reader-get-ns) (mu:sy-name symbol)))
           ((:lambda (name)
               (mu:intern
                (prelude:%reader-get-ns)
                name
                value))
            (mu:sy-name symbol)))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; parsers
;;;

;;;
;;; number reader
;;;
(mu:intern :prelude "parse-integer"
  (:lambda (digits base)
    (:if (prelude:stringp digits)
         (:if (prelude:fixnump base)
              (:if (prelude:zerop (mu:sv-len digits))
                   ()
                   ((:lambda (sign)
                       ((:lambda (integer)
                           (:if (prelude:fixnump integer)
                                (:if (prelude:null sign)
                                     integer
                                     (mu:fx-mul sign integer))
                                ()))
                        (mu:cdr
                         (mu:fix
                          (:lambda (loop)
                             (:if (prelude:null loop)
                                  ()
                                  ((:lambda (index acc)
                                      (:if (mu:fx-lt (mu:fx-sub (mu:sv-len digits) 1) index)
                                           loop
                                           ((:lambda (n)
                                               (:if (prelude:null n)
                                                    ()
                                                    (:if (mu:fx-lt (mu:fx-sub base 1) n)
                                                         ()
                                                         `(,(mu:fx-add 1 index) ,@(mu:fx-add n (mu:fx-mul acc base))))))
                                            (prelude:string-position (prelude:schar digits index) "0123456789abcdef"))))
                                   (mu:car loop)
                                   (mu:cdr loop))))
                          (:if (prelude:fixnump sign)
                               '(1 . 0)
                               '(0 . 0))))))
                    ((:lambda (digit)
                        (:if (mu:eq #\- digit)
                             -1
                             (:if (mu:eq #\+ digit)
                                  1
                                  ())))
                     (prelude:schar digits 0))))
              (prelude:raise base 'prelude:parse-integer "not a fixnum"))
         (prelude:raise digits 'prelude:parse-integer "not a string"))))

(mu:intern :prelude "parse-float"
  (:lambda (str)
    ((:lambda (stream)
       ((:lambda (float)
          (:if (prelude:floatp float)
               float
               ()))
          (mu:read stream () ())))
       (mu:open :string :input str))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; read macros
;;;

;;;
;;; sharpsign readers
;;;
(mu:intern :prelude "%read-sharp-char"
   (:lambda (ch stream)
      (prelude:read-char stream () ())))

(mu:intern :prelude "%read-sharp-comment"
   (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\|)
                      ((:lambda (ch)
                         ((:lambda (ch)
                            (:if (prelude:null ch)
                                 (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                                 (:if (mu:eq ch #\#)
                                      stream
                                      (prelude:null loop))))
                          (prelude:read-char stream () ())))
                       (:if (prelude:null ch)
                            (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                            (prelude:read-char stream () ())))
                      (prelude:null loop)))
               (:if (prelude:null ch)
                    (prelude:raise stream 'prelude:%read-sharp-comment "unexpected end of file")
                    (prelude:read-char stream () ())))))
       ())))

(mu:intern :prelude "%read-sharp-vector"
   (:lambda (ch stream)
      ((:lambda (type)
          (:if (prelude:keywordp type)
               (mu:vector type (prelude:%read-list #\( stream))
               (prelude:raise type 'prelude:%read-sharp-vector "not a type keyword")))
          (prelude:%read stream))))

(mu:intern :prelude "%read-byte-bits"
   (:lambda (stream)
     (mu:cdr
      (mu:fix
       (:lambda (loop)
         ((:lambda (nbits byte)
            (:if (prelude:zerop nbits)
                 loop
                 ((:lambda (ch)                   
                    (:if (prelude:null ch)
                         (mu:cons 0 `(,(mu:fx-sub 8 nbits) ,@(mu:ash byte nbits)))
                         ((:lambda (syntax)
                            (:if (prelude:%orf (mu:eq :tmacro syntax) (mu:eq :wspace syntax))
                                 ((:lambda ()
                                    (:if (mu:eq :tmacro syntax)
                                         (prelude:unread-char ch stream)
                                         ())
                                    (mu:cons 0 `(,(mu:fx-sub 8 nbits) ,@(mu:ash byte nbits)))))
                                 (:if (prelude:%orf (mu:eq #\0 ch) (mu:eq #\1 ch))
                                      `(,(prelude:1- nbits)
                                        ,@(mu:logor
                                           (:if (mu:eq #\1 ch) 1 0)
                                           (mu:fx-mul byte 2)))
                                      (prelude:raise ch 'prelude:%read-sharp-bit-vector "not a bit designator"))))
                          (prelude:%read-char-syntax ch))))
                  (prelude:read-char stream () ()))))
          (mu:car loop)
          (mu:cdr loop)))
        '(8 . 0)))))
             
(mu:intern :prelude "%read-sharp-bit-vector"
   (:lambda (ch stream)
     (mu:fix
       (:lambda (loop)
         (:if (prelude:vectorp loop)
              loop
              ((:lambda (byte-descriptor)
                 (:if (prelude:consp byte-descriptor)
                      ((:lambda (length byte)
                         (prelude:%make-vector
                          (mu:vector :byte (prelude:reverse `(,byte ,@loop)))
                          ()
                          `(,(mu:fx-add length (mu:fx-mul 8 (mu:length loop))))))
                       (mu:car byte-descriptor)
                       (mu:cdr byte-descriptor))
                       `(,byte-descriptor ,@loop)))
               (prelude:%read-byte-bits stream))))
       ())))

(mu:intern :prelude "%read-sharp-dot"
   (:lambda (ch stream)
      (mu:eval (prelude:compile (prelude:read stream () ())))))

(mu:intern :prelude "%read-sharp-symbol"
   (:lambda (ch stream)
     ((:lambda (symbol)
          (:if (mu:eq :symbol (mu:type-of symbol))
               (mu:symbol (mu:sy-name symbol))
               (prelude:raise symbol 'prelude:%read-sharp-symbol "not a symbol")))
       (prelude:%read-atom ch stream))))

(mu:intern :prelude "%read-sharp-number"
  (:lambda (base stream)
    (mu:fix
      (:lambda (loop)
        (:if (prelude:numberp loop)
             loop
             ((:lambda (ch)
                (:if (prelude:null ch)
                     (:if (mu:eq base #\x)
                          (prelude:parse-integer (mu:get-str prelude:%reader-stream%) 16)
                          (:if (mu:eq base #\b)
                               (prelude:parse-integer (mu:get-str prelude:%reader-stream%) 2)
                               (prelude:parse-integer (mu:get-str prelude:%reader-stream%) 10)))
                     ((:lambda (syntax-type)
                        (:if (mu:eq syntax-type :const)
                             ((:lambda ()
                                (prelude:write-char ch prelude:%reader-stream%)
                                (prelude:null loop)))
                             ((:lambda ()
                                (prelude:unread-char ch stream)
                                (:if (mu:eq base #\x)
                                     (prelude:parse-integer (mu:get-str prelude:%reader-stream%) 16)
                                     (:if (mu:eq base #\b)
                                          (prelude:parse-integer (mu:get-str prelude:%reader-stream%) 2)
                                          (prelude:parse-integer (mu:get-str prelude:%reader-stream%) 10)))))))
                     (prelude:%read-char-syntax ch))))
              (prelude:read-char stream () ()))))
      ())))

(mu:intern :prelude "%read-sharp"
  (:lambda (ch stream)
    ((:lambda (ch sharp-table)
       (mu:apply
        (mu:sy-val (mu:cdr (prelude:%assoc ch sharp-table)))
        `(,ch ,stream)))
     (prelude:read-char stream () ())
     '((#\| . prelude:%read-sharp-comment)
       (#\( . prelude:%read-sharp-vector)
       (#\* . prelude:%read-sharp-bit-vector)
       (#\. . prelude:%read-sharp-dot)
       (#\\ . prelude:%read-sharp-char)
       (#\b . prelude:%read-sharp-number)
       (#\x . prelude:%read-sharp-number)
       (#\d . prelude:%read-sharp-number)
       (#\: . prelude:%read-sharp-symbol)))))

;;;
;;; list reader
;;;
(mu:intern :prelude "%read-list-eol%" (mu:symbol "eol"))

(mu:intern :prelude "%read-list-eol"
   (:lambda (ch stream)
      prelude:%read-list-eol%))

(mu:intern :prelude "%read-list"
   (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         ((:lambda (el)
            (:if (mu:eq el prelude:%read-list-eol%)
                 loop
                 (:if (mu:eq :symbol (mu:type-of el))
                      (:if (mu:eq (mu:sy-name el) ".")
                           ((:lambda (dotted)
                              (:if (mu:eq (prelude:%read stream) prelude:%read-list-eol%)
                                   ((:lambda ()
                                      (prelude:unread-char #\) stream)
                                      dotted))
                                   (prelude:raise () 'prelude:%read-list "malformed dotted list")))
                            (prelude:foldr mu:cons (prelude:%read stream) loop))
                           `(,@loop ,el))
                      `(,@loop ,el))))
          (prelude:%read stream)))
     ())))

;;;
;;; string reader
;;;
(mu:intern :prelude "%read-string"
   (:lambda (ch stream)
     (mu:fix
      (:lambda (loop)
        (:if (prelude:stringp loop)
             loop
             ((:lambda (ch)
                (:if (mu:eq ch #\")
                     (prelude:get-output-stream-string prelude:%reader-stream%)
                     ((:lambda ()
                        (prelude:write-char ch prelude:%reader-stream%)
                        (prelude:null loop)))))
              (prelude:read-char stream () ()))))
        ())))

;;;
;;; quote reader
;;;
(mu:intern :prelude "%read-quote"
  (:lambda (ch stream)
     `(:quote ,(prelude:%read stream))))

;;;
;;; line comment
;;;
(mu:intern :prelude "%read-line-comment"
    (:lambda (ch stream)
      (mu:fix
       (:lambda (loop)
         (:if (prelude:streamp loop)
              loop
              ((:lambda (ch)
                 (:if (mu:eq ch #\linefeed)
                      stream
                      (prelude:null loop)))
               (prelude:read-char stream () ()))))
       ())))

;;;
;;; read macros
;;;
(mu:intern :prelude "%read-macro"
  (:lambda (ch stream)
     ((:lambda (macro-table)
       (mu:apply
            (mu:sy-val (mu:cdr (prelude:%assoc ch macro-table)))
            `(,ch ,stream)))
       '((#\" . prelude:%read-string)
         (#\# . prelude:%read-sharp)
         (#\' . prelude:%read-quote)
         (#\` . prelude:%qq-reader)
         (#\( . prelude:%read-list)
         (#\) . prelude:%read-list-eol)
         (#\; . prelude:%read-line-comment)))))

;;;
;;; get/set-macro-character
;;;
(mu:intern :prelude "get-macro-character"
   (:lambda (char)
     (:if (prelude:charp char)
          ((:lambda (macro-symbol)
              (:if macro-symbol
                   (prelude:symbol-value macro-symbol)
                   ()))
           (mu:ns-find prelude:%char-macro-ns% (mu:vector :char `(,char))))
          (prelude:raise char 'prelude:get-macro-character "not a character"))))

(mu:intern :prelude "set-macro-character"
   (:lambda (char fn non-terminating)
      (:if (prelude:charp char)
         (:if (prelude:functionp fn)
              (mu:intern prelude:%char-macro-ns% (mu:vector :char `(char)) `(,fn ,@non-terminating))
              (prelude:raise fn 'prelude:set-macro-character "not a function"))
         (prelude:raise char 'prelude:set-macro-character "not a character"))
      :t))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; reader
;;;
(mu:intern :prelude "%eof%" (mu:symbol "eof"))
(mu:intern :prelude "%reader-stream%" (mu:open :string :output ""))
(mu:intern :prelude "%reader-current-ns%" (mu:open :string :bidir "prelude"))

(mu:intern :prelude "%in-namespace"
   (:lambda (ns)
      ((:lambda ()
          (mu:make-ns ns)
          (prelude:%reader-set-ns ns)
          ns))))

(mu:intern :prelude "%reader-get-ns"
   (:lambda ()
     ((:lambda (ns)
        (mu:write ns () prelude:%reader-current-ns%)
        (mu:keyword ns))
      (prelude:get-output-stream-string prelude:%reader-current-ns%))))

(mu:intern :prelude "%reader-set-ns"
   (:lambda (ns)
      (prelude:get-output-stream-string prelude:%reader-current-ns%)
      (mu:write (mu:sy-name ns) () prelude:%reader-current-ns%)))

;;;
;;; readtable
;;;
(mu:intern :prelude "%read-char-syntax"
   (:lambda (ch)
     ((:lambda (read-table) (mu:cdr (prelude:%assoc ch read-table)))
      '((#\return . :wspace) (#\linefeed . :wspace) (#\page . :wspace)
        (#\tab . :wspace) (#\space . :wspace)
        (#\0 . :const)  (#\1 . :const)  (#\2 . :const)  (#\3 . :const)
        (#\4 . :const)  (#\5 . :const)  (#\6 . :const)  (#\7 . :const)
        (#\8 . :const)  (#\9 . :const)  (#\: . :const)  (#\< . :const)
        (#\> . :const)  (#\= . :const)  (#\? . :const)  (#\! . :const)
        (#\@ . :const)  (#\; . :tmacro) (#\" . :tmacro) (#\# . :macro)
        (#\' . :tmacro) (#\( . :tmacro) (#\) . :tmacro) (#\` . :tmacro)
        (#\, . :tmacro) (#\\ . :escape) (#\| . :mescape) (#\a . :const)
        (#\b . :const)  (#\c . :const)  (#\d . :const)  (#\e . :const)
        (#\f . :const)  (#\g . :const)  (#\h . :const)  (#\i . :const)
        (#\j . :const)  (#\k . :const)  (#\l . :const)  (#\m . :const)
        (#\n . :const)  (#\o . :const)  (#\p . :const)  (#\q . :const)
        (#\r . :const)  (#\s . :const)  (#\t . :const)  (#\v . :const)
        (#\w . :const)  (#\x . :const)  (#\y . :const)  (#\z . :const)
        (#\[ . :const)  (#\] . :const)  (#\$ . :const)  (#\* . :const)
        (#\{ . :const)  (#\} . :const)  (#\+ . :const)  (#\- . :const)
        (#\/ . :const)  (#\~ . :const)  (#\. . :const)  (#\% . :const)
        (#\& . :const)  (#\^ . :const)  (#\_ . :const)  (#\a . :const)
        (#\b . :const)  (#\c . :const)  (#\d . :const)  (#\e . :const)
        (#\f . :const)  (#\g . :const)  (#\h . :const)  (#\i . :const)
        (#\j . :const)  (#\k . :const)  (#\l . :const)  (#\m . :const)
        (#\n . :const)  (#\o . :const)  (#\p . :const)  (#\q . :const)
        (#\r . :const)  (#\s . :const)  (#\t . :const)  (#\u . :const)
        (#\v . :const)  (#\w . :const)  (#\x . :const)  (#\y . :const)
        (#\z . :const)))))

(mu:intern :prelude "%read-number"
  (:lambda (atom)
    ((:lambda (fx)
        (:if fx
             fx
             ((:lambda (fl)
                (:if fl
                     fl
                     ()))
             (prelude:parse-float atom))))
      (prelude:parse-integer atom 10))))

;;;
;;; atom reader
;;;
(mu:intern :prelude "%read-atom"
  (:lambda (ch stream)
    (prelude:write-char ch prelude:%reader-stream%)
    (mu:fix
     (:lambda (loop)
       (:if (prelude:consp loop)
            loop
            ((:lambda (ch)
               (:if (mu:eq prelude:%eof% ch)
                    loop
                    ((:lambda (syntax-type)
                       (:if (mu:eq :const syntax-type)
                            ((:lambda ()
                               (prelude:write-char ch prelude:%reader-stream%)
                               (prelude:null loop)))
                            ((:lambda ()
                               (prelude:unread-char ch stream)
                               loop))))
                     (prelude:%read-char-syntax ch))))
             (prelude:read-char stream () prelude:%eof%))))
     ())
    ((:lambda (token)
       ((:lambda (number)
          (:if number
               number
               (prelude:%symbol-macro-expand (prelude:%read-symbol token))))
        (prelude:%read-number token)))
     (prelude:get-output-stream-string prelude:%reader-stream%))))

;;;
;;; parser
;;;
(mu:intern :prelude "%read-dispatch"
  (:lambda (ch stream)
    ((:lambda (dispatch-table)
       (mu:apply
        (mu:sy-val (mu:cdr (prelude:%assoc (prelude:%read-char-syntax ch) dispatch-table)))
        `(,ch ,stream)))
     '((:const   . prelude:%read-atom)
       (:escape  . prelude:%read-atom)
       (:macro   . prelude:%read-macro)
       (:tmacro  . prelude:%read-macro)
       (:mescape . prelude:%read-atom)))))

;;;
;;; consume whitespace and comments
;;;
(mu:intern :prelude "%read-consume-ws"
   (:lambda (stream)
    (mu:fix
     (:lambda (loop)
       (:if (prelude:%orf (prelude:streamp loop) (prelude:charp loop))
            loop
            ((:lambda (ch)
               (:if (prelude:null ch)
                    stream
                    (:if (mu:eq :wspace (prelude:%read-char-syntax ch))
                         (prelude:null loop)
                         (:if (mu:eq ch #\#)
                              ((:lambda (ch)
                                 (:if (mu:eq ch #\|)
                                      ((:lambda ()
                                          (prelude:%read-sharp-comment ch stream)
                                          (prelude:null loop)))
                                      #\#))
                                (prelude:peek-char stream () ()))
                              (:if (mu:eq ch #\;)
                                   ((:lambda ()
                                       (prelude:%read-line-comment ch stream)
                                       (prelude:null loop)))
                                   ch)))))
             (prelude:read-char stream () ()))))
     ())))

;;;
;;; recursive reader
;;;
(mu:intern :prelude "%read"
  (:lambda (stream)
    ((:lambda (ch)
       (:if (prelude:null ch)
            prelude:%eof%
            ((:lambda (macro)
               (:if macro
                    (prelude:apply (mu:car macro) `(,stream ,ch))
                    (prelude:%read-dispatch ch stream)))
             (prelude:get-macro-character ch))))
     (prelude:%read-consume-ws stream))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; repl
;;;
(mu:intern :prelude "repl"
   (:lambda ()
     (mu:fix
       (:lambda (loop)
         (prelude:with-exception
             (:lambda (ex)
               (prelude:break ex)
               (prelude:null loop))
           (:lambda ()
             (prelude:format :t "prelude> " ())
             (mu:flush mu:std-out)
             ((:lambda (form)
                (:if (mu:eq form prelude:%eof%)
                     loop
                     ((:lambda (value)
                        (prelude:format :t "[~A] ~S~%" `(,(prelude:type-of value) ,value))
                        (prelude:null loop))
                      (mu:eval (prelude:compile form)))))
              (prelude:read :t () prelude:%eof%)))))
       ())))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;
(mu:intern :prelude "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))

;;;
;;; utilities
;;;
(mu:intern :prelude "%write-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         mu:std-out
         (:if (prelude:streamp designator)
              designator
              (prelude:raise designator
                          'prelude:%write-stream-designator
                          "not a stream designator")))))

(mu:intern :prelude "%read-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         mu:std-in
         (:if (mu:eq :t designator)
              mu:std-in
              (:if (prelude:streamp designator)
                   designator
                   (prelude:raise designator
                               'prelude:%read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(mu:intern :prelude "make-string-stream"
   (:lambda (dir init)
      (:if (prelude:keywordp dir)
           (:if (prelude:stringp init)
                (:if (mu:eq dir :input)
                     (mu:open :string :input init)
                     (:if (mu:eq dir :output)
                          (mu:open :string :output init)
                          (:if (mu:eq dir :bidir)
                               (mu:open :string :bidir init)
                               (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))))
                (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))
           (prelude:raise init 'prelude:make-string-stream "not a string initializer"))))

(mu:intern :prelude "open"
  (:lambda (dir path)
     (:if (prelude:stringp path)
          (:if (prelude:keywordp dir)
               (:if (mu:eq dir :input)
                    (mu:open :file :input path)
                    (:if (mu:eq dir :output)
                         (mu:open :file :output path)
                         (prelude:raise dir 'prelude:open "not a direction keyword")))
               (prelude:raise path 'prelude:open "not a path string"))
          (prelude:raise dir 'prelude:open "not a direction keyword"))))

(mu:intern :prelude "close"
   (:lambda (stream)
      (:if (prelude:streamp stream)
           (mu:close stream)
           (prelude:raise stream 'prelude:close "not a stream"))))

(mu:intern :prelude "with-open-stream"
   (:lambda (type dir specifier fn)
     (:if (prelude:functionp fn)
          (:if (prelude:stringp specifier)
               (:if (prelude:%orf (mu:eq :file type)
                               (mu:eq :string type))
                    (:if (prelude:%orf (mu:eq :input dir)
                                    (mu:eq :output dir))
                         ((:lambda (stream)
                             (mu:with-ex
                              (:lambda (ex) (mu:close stream) ex)
                              (:lambda () (prelude:apply fn `(,stream)))))
                          (mu:open type dir specifier))
                         (prelude:raise dir 'prelude:with-open-stream "not a direction keyword"))
                    (prelude:raise type 'prelude:with-open-stream "not a type keyword"))
               (prelude:raise path 'prelude:with-open-stream "not a stream specifier"))
          (prelude:raise dir 'prelude:with-open-stream "not a function"))))

;;;
;;; chars, bytes, and unread
;;;
(mu:intern :prelude "write-char"
   (:lambda (ch designator)
     (mu:wr-char ch (prelude:%write-stream-designator designator))))

(mu:intern :prelude "write-byte"
   (:lambda (byte designator)
     (mu:wr-byte byte (prelude:%write-stream-designator designator))))

(mu:intern :prelude "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:rd-char stream error-eofp eof-value))
     (prelude:%read-stream-designator designator))))

(mu:intern :prelude "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (mu:rd-byte stream error-eofp eof-value))
      (prelude:%read-stream-designator designator))))

(mu:intern :prelude "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:un-char
        (prelude:read-char stream error-eofp eof-value)
        stream))
     (prelude:%read-stream-designator designator))))

(mu:intern :prelude "unread-char"
   (:lambda (ch designator)
     (:if (prelude:charp ch)
          (mu:un-char ch (prelude:%write-stream-designator designator))
          (prelude:raise ch 'prelude:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(mu:intern :prelude "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (mu:eq prelude:%eof% form)
                (:if eof-error
                     (prelude:raise stream 'prelude:read "early end of file")
                     eof-value)
                (:if (mu:eq form prelude:%read-list-eol%)
                     (prelude:raise stream 'prelude:read "unmatched close parenthesis")
                     form)))
         (prelude:%read stream)))
      (prelude:%read-stream-designator designator))))

(mu:intern :prelude "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (mu:eq value prelude:%eof%)
                eof-value
                (mu:get-str line)))              
         (mu:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (mu:eq ch prelude:%eof%)
                    prelude:%eof%
                    (:if (mu:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (mu:wr-char ch line)
                            (prelude:null loop))))))
             (prelude:read-char stream eof-error-p prelude:%eof%)))
         ())))
      (mu:open :string :output ""))))

(mu:intern :prelude "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (prelude:%andf (prelude:vectorp object) (prelude:%prelude-type-p object))
               (prelude:%vector-write object escape stream)
               (mu:write object escape stream)))
      (prelude:%read-stream-designator designator))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(mu:intern :prelude "stringp"
   (:lambda (string)
      (:if (mu:eq (mu:type-of string) :vector)
           (mu:eq :char (mu:sv-type string))
           (:if (prelude:%prelude-type-p string)
                (:if (mu:eq "vector" (mu:sv-ref (mu:st-vec string) 0))
                     (mu:eq :char (mu:sv-type (prelude:%vector-prop :base string)))
                     ())
                ()))))

(mu:intern :prelude "get-output-stream-string"
   (:lambda (stream)
      (mu:get-str stream)))

(mu:intern :prelude "schar"
  (:lambda (str index)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump index)
               (mu:sv-ref str index)
               (prelude:raise index 'prelude:schar "not a string"))
          (prelude:raise str 'prelude:schar "not a string"))))

(mu:intern :prelude "string="
   (:lambda (str-1 str-2)
      (:if (prelude:stringp str-1)
           (:if (prelude:stringp str-2)
                (:if (mu:eq str-1 str-2)
                     :t
                     (:if (mu:eq (mu:sv-len str-1) (mu:sv-len str-2))
                          (mu:fix
                           (:lambda (nth)
                              (:if (prelude:numberp nth)
                                   (:if (mu:eq nth (mu:sv-len str-1))
                                        :t
                                        (:if (mu:eq (mu:sv-ref str-1 nth) (mu:sv-ref str-2 nth))
                                             (mu:fx-add 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (prelude:raise str-2 'prelude:string= "not a string"))
           (prelude:raise str-1 'prelude:string= "not a string"))))

(mu:intern :prelude "%string-write"
   (:lambda (string escape stream)
      (:if (mu:eq :vector (mu:type-of string))
           (mu:write string escape stream)
           ((:lambda (length)
               (:if escape
                    (mu:write #\" () stream)
                    ())
               (mu:fix
                (:lambda (index)
                   (:if (mu:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref string index) () stream)
                            (prelude:1+ index)))
                        index))
                0)
               (:if escape
                    (mu:write #\" () stream)
                    ()))
            (prelude:%vector-prop :length string)))
      ()))

;;;
;;; string-position
;;;
(mu:intern :prelude "string-position"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (mu:sv-len str))
                   ()
                   ((:lambda (len)
                       (mu:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (mu:fx-lt (mu:fx-sub len 1) loop)
                                     ()
                                     (:if (mu:eq ch (mu:sv-ref str loop))
                                          loop
                                          (mu:fx-add 1 loop)))
                                loop))
                        0))
                    (mu:sv-len str)))
              (prelude:raise str 'prelude:string-position "not a string"))
         (prelude:raise ch 'prelude:string-position "not a char"))))

;;;
;;; string-find
;;;
(mu:intern :prelude "string-find"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (mu:sv-len str))
                   ()
                   ((:lambda (len)
                       (mu:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (mu:fx-lt (mu:fx-sub len 1) loop)
                                     ()
                                     (:if (mu:eq ch (mu:sv-ref str loop))
                                          ch
                                          (mu:fx-add 1 loop)))
                                loop))
                        0))
                    (mu:sv-len str)))
              (prelude:raise str 'prelude:string-find "not a string"))
         (prelude:raise ch 'prelude:string-find "not a char"))))

;;;
;;; string construction
;;;
(mu:intern :prelude "string"
   (:lambda (designator)
      (:if (prelude:stringp designator)
         designator
         (:if (mu:eq :symbol (mu:type-of designator))
              (mu:sy-name designator)
              (:if (prelude:charp designator)
                   (mu:vector :char `(,designator))
                   (prelude:raise designator 'prelude:string "not a string designator"))))))

(mu:intern :prelude "string-append"
   (:lambda (list)
      (:if (prelude:listp list)
           ((:lambda (stream)
               (mu:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (prelude:stringp str)
                                     (mu:write str () stream)
                                     (prelude:raise str 'prelude:string-append "is not a string")))
                             (mu:car list))
                            (mu:cdr list)))
                        ()))
                list)
               (mu:get-str stream))
            (mu:open :string :output ""))
           (prelude:raise list 'prelude:string "is not a list"))))

(mu:intern :prelude "substr"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    ((:lambda (substr)
                        (mu:fix
                         (:lambda (nth)
                            (:if (prelude:numberp nth)
                                 (:if (mu:eq nth (mu:sv-len str))
                                      ()
                                      (:if (mu:fx-lt nth (mu:fx-add 1 end))
                                           ((:lambda ()
                                               (mu:write (mu:sv-ref str nth) () substr)
                                               (mu:fx-add 1 nth)))
                                           ()))
                                 nth))
                         start)
                        (mu:get-str substr))
                     (mu:open :string :output ""))
                    (prelude:raise end 'prelude:substr "end is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "start is not a fixnum"))))

#|
(mu:intern :prelude "substr-"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    (prelude:slice str start end)
                    (prelude:raise end 'prelude:substr "is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "is not a fixnum"))))
|#

;;;
;;; read line
;;;
(mu:intern :prelude "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (prelude:streamp stream)
           ((:lambda (sstream)
               (mu:fix
                (:lambda (loop)
                   (:if (prelude:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (mu:eq ch #\linefeed)
                                 (mu:get-str sstream)
                                 ((:lambda ()
                                     (mu:wr-char ch sstream)
                                     (prelude:null loop)))))
                         (prelude:read-char stream eof-error eof-value))))
                ()))
            (mu:open :string :output ""))
           (prelude:raise str 'prelude:read-line "is not a stream"))))

;;;
;;; read from string
;;;
(mu:intern :prelude "%read-string%" (mu:open :string :bidir ""))
(mu:intern :prelude "read-string"
   (:lambda (str eof-error eof-value)
      (:if (prelude:stringp str)
           ((:lambda ()
              (mu:write str () prelude:%read-string%) 
              (mu:read prelude:%read-string% eof-error eof-value)))
           (prelude:raise str 'prelude:read-string "is not a string"))))

;;;
;;; write string
;;;
(mu:intern :prelude "write-string"
   (:lambda (str designator)
      (:if (prelude:stringp str)
           ((:lambda (stream)
              (mu:write str () stream))
            (prelude:%read-stream-designator designator))
           (prelude:raise str 'prelude:write-string "is not a string"))))

;;;
;;; write line
;;;
(mu:intern :prelude "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (prelude:write-string str stream)
         (prelude:write-char #\linefeed stream))
       (prelude:%write-stream-designator designator))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol macros
;;;

;;;
;;; symbol readers
;;;
(mu:intern :prelude "%read-keywordp"
  (:lambda (name)
      (mu:eq #\: (prelude:schar name 0))))

(mu:intern :prelude "%read-symbol-scope"
  (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             :extern
             (:if (mu:eq #\: (prelude:schar name (mu:fx-add 1 colon)))
                  :intern
                  :extern)))
        (prelude:string-position #\: name))))

(mu:intern :prelude "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon)
       (:if (prelude:null colon)
            ()
            (mu:keyword (prelude:substr name 0 (mu:fx-sub colon 1)))))
     (prelude:string-position #\: name))))

(mu:intern :prelude "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (prelude:null colon)
             name
             (prelude:substr name
                          (:if (mu:eq #\: (prelude:schar name (mu:fx-add 1 colon)))
                               (mu:fx-add 2 colon)
                               (mu:fx-add 1 colon))
                          (:if (mu:eq #\: (prelude:schar name (mu:fx-add 1 colon)))
                               (mu:fx-add colon (mu:fx-sub (mu:sv-len name) colon))
                               (mu:fx-add colon (mu:fx-sub (mu:sv-len name) (mu:fx-add 1 colon)))))))
     (prelude:string-position #\: name))))

(mu:intern :prelude "%read-symbol"
   (:lambda (symbol)
     (:if (prelude:%read-keywordp symbol)
          (prelude:keyword (prelude:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (prelude:null ns)
                  (mu:untern () name)
                  (mu:untern ns name)))
           (prelude:%read-symbol-ns symbol)
           (prelude:%read-symbol-name symbol)))))

;;;
;;; symbol macros
;;;
(mu:intern :prelude "%symbol-macro-expand"
   (:lambda (symbol)
     (:if (prelude:%orf (prelude:null symbol) (prelude:keywordp symbol))
          symbol
          (:if (prelude:uninternedp symbol)
               ((:lambda (sym)
                  (:if sym
                       (mu:eval (prelude:symbol-value sym))
                       symbol))
                (prelude:%get-symbol-macro symbol))
               symbol))))

(mu:intern :prelude "%get-symbol-macro"
   (:lambda (symbol)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:ns-find prelude:%symbol-macro-ns% (prelude:symbol-name symbol))
           (prelude:raise symbol 'prelude:%get-symbol-macro "not a symbol"))))

(mu:intern :prelude "define-symbol-macro"
   (:lambda (symbol form)
      (:if (mu:eq :symbol (mu:type-of symbol))
           (mu:intern prelude:%symbol-macro-ns% (mu:sy-name symbol) form)
           (prelude:raise symbol 'prelude:define-symbol-macro "not a symbol"))))

;;; (prelude:define-symbol-macro 't :t)
;;; (prelude:define-symbol-macro 'nil :nil)
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  symbol predicates
;;;
(mu:intern :prelude "boundp" mu:boundp)

(mu:intern :prelude "fboundp"
  (:lambda (symbol)
    (:if (prelude:boundp symbol)
         (prelude:functionp (mu:sy-val symbol))
         ())))

(mu:intern :prelude "uninternedp"
   (:lambda (sym)
      (prelude:null (prelude:symbol-ns sym))))

(mu:intern :prelude "keywordp"
   (:lambda (t)
      (mu:eq (prelude:type-of t) 'keyword)))

(mu:intern :prelude "symbolp"
   (:lambda (t)
      (prelude:%orf (prelude:%orf (prelude:keywordp t) (mu:eq () t))
                (mu:eq 'symbol (prelude:type-of t)))))

;;;
;;; symbol components
;;;
(mu:intern :prelude "symbol-name" mu:sy-name)

(mu:intern :prelude "symbol-ns"
   (:lambda (sym)
      (mu:sy-ns sym)))

(mu:intern :prelude "symbol-value" mu:sy-val)

(mu:intern :prelude "keyword"
   (:lambda (name)
      (:if (prelude:stringp name)
           (:if (mu:fx-lt (mu:sv-len name) 8)
                (mu:keyword name)
                (prelude:%make-type "keyword" `(,(mu:cons :name name))))
           (prelude:raise name 'prelude:keyword "not a legal name"))))

;;;
;;; utilities
;;;
(mu:intern :prelude "%gensym-counter"
   (mu:open :string :bidir "0"))

(mu:intern :prelude "gensym"
   (:lambda ()
      ((:lambda (counter)
          (mu:write (prelude:1+ counter) () prelude:%gensym-counter)
          (mu:symbol (prelude:format () "g~X" `(,counter))))
       (mu:read prelude:%gensym-counter () ()))))

(mu:intern :prelude "genkeyword"
   (:lambda (prefix)
      (:if (prelude:%orf (prelude:null prefix) (prelude:charp prefix))
           ((:lambda (counter)
              (mu:write (prelude:1+ counter) () prelude:%gensym-counter)
              (mu:keyword
               (:if (prelude:charp prefix)
                    (prelude:format () "~A~X" `(,prefix ,counter))
                    (prelude:format () "<~X>" `(,counter)))))
            (mu:read prelude:%gensym-counter () ()))
           (prelude:raise name 'prelude:genkeyword "not a legal prefix"))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; time function
;;;
(mu:intern :prelude "time"
   (:lambda (fn args)
      (:if (prelude:functionp fn)
           (:if (prelude:listp args)
                ()
                (prelude:raise args 'prelude:time "is not a list"))
           (prelude:raise fn 'prelude:time "is not a function"))
     ((:lambda (start-vec time-value stop-vec)
         ((:lambda (time-delta value type-diffs)
             (prelude:format :t "Evaluation consumed ~A usec runtime~%" `(,time-delta))
             (prelude:format :t "           consed ~A~%" `(,type-diffs))
             value)
            (mu:car time-value)
            (mu:cdr time-value)
            (mu:fix
             (:lambda (loop)
                (:if (mu:eq :vector (mu:type-of loop))
                     loop
                     ((:lambda (list n)
                        ((:lambda (offset)
                            (:if (mu:fx-lt offset (mu:sv-len start-vec))
                                 ((:lambda (type total alloc free)
                                     (:if (prelude:zerop alloc)
                                           `(,list ,@(mu:fx-add 1 n))
                                           ((:lambda (type-stats)
                                               (mu:cons `(,@list ,@type-stats) (mu:fx-add 1 n)))
                                            `(,type ,total ,alloc ,free))))
                                  (mu:sv-ref start-vec offset)
                                  (mu:fx-sub (mu:sv-ref stop-vec (mu:fx-add offset 1))
                                             (mu:sv-ref start-vec (mu:fx-add offset 1)))
                                  (mu:fx-sub (mu:sv-ref stop-vec (mu:fx-add offset 2))
                                             (mu:sv-ref start-vec (mu:fx-add offset 2)))
                                  (mu:fx-sub (mu:sv-ref stop-vec (mu:fx-add offset 3))
                                             (mu:sv-ref start-vec (mu:fx-add offset 3))))
                                 (mu:vector :t list)))
                          (mu:fx-mul n 4)))
                      (mu:car loop)
                      (mu:cdr loop))))
             '(() . 1))))
        (mu:hp-stat)
        ((:lambda (start)
           ((:lambda (value)
              `(,(mu:fx-sub (mu:proc-tm) start) ,@value))
            (prelude:apply fn args)))
        (mu:proc-tm))
       (mu:hp-stat))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; types
;;;
(mu:intern :prelude "type-of"
   (:lambda (value)
      ((:lambda (type)
          (:if (mu:eq type :struct)
               (:if (mu:eq :<type> (mu:st-type value))
                    (mu:untern () (mu:sv-ref (mu:st-vec value) 0))
                    (mu:st-type value))
               (:if (prelude:stringp value)
                    'string
                    (mu:untern () (mu:sy-name type)))))
       (mu:type-of value))))

;;;
;;; typep
;;;
(mu:intern :prelude "%typespec-map%"
   `((asyncid :asyncid)
     (bit-vector (satisfies prelude:bit-vector-p))
     (char (satisfies prelude:charp))
     (function (satisfies prelude:functionp))
     (map :map)
     (number (or (satisfies prelude:floatp) integer))
     (integer (satisfies prelude:fixnump))
     (sequence (or (satisfies prelude:vectorp) (satisfies prelude:stringp) (satisfies prelude:listp)))
     (stream (satisfies prelude:streamp))
     (string (satisfies prelude:stringp))
     (struct (satisifes prelude:structp))
     (symbol (or (satisfies prelude:symbolp) (satisfies prelude:keywordp)))))

(mu:intern :prelude "%compound-typespec-map%"
   `(,(mu:cons 'satisfies
         (:lambda (value spec-list)
            (:if (mu:eq 1 (mu:length spec-list))
                 ((:lambda (predicate-symbol)
                    (:if (prelude:symbolp predicate-symbol)
                         (:if (mu:boundp predicate-symbol)
                              (prelude:apply (mu:sy-val predicate-symbol) `(,value))
                              (prelude:raise predicate 'prelude:typep "satisfies typespec format"))
                         (prelude:raise predicate 'prelude:typep "satisfies typespec format")))
                  (mu:car spec-list))
                 (prelude:raise spec-list 'prelude:typep "satisfies typespec format"))))
     ,(mu:cons 'not (:lambda (value spec-list)
                       (:if (mu:eq 1 (mu:length spec-list))
                            (prelude:not (prelude:typep value (mu:car spec-list)))
                            (prelude:raise spec-list 'prelude:typep "no typespec format"))))
     ,(mu:cons 'and (:lambda (value spec-list)
                       (:if (prelude:null spec-list)
                            :t
                            (prelude:foldl
                             (:lambda (elt acc)
                                (prelude:%andf acc (prelude:typep value elt)))
                             :t
                             spec-list))))
     ,(mu:cons 'or (:lambda (value spec-list)
                     (:if (prelude:null spec-list)
                          ()
                          (prelude:foldl
                           (:lambda (elt acc)
                             (prelude:%orf acc (prelude:typep value elt)))
                           :t
                           spec-list))))))

(mu:intern :prelude "typep"
   (:lambda (value typespec)
     (:if (prelude:keywordp typespec)
          (mu:eq typespec (mu:type-of value))
          (:if (prelude:keywordp typespec)
               (mu:eq typespec (prelude:type-of value))
               (:if (prelude:symbolp typespec)
                    ((:lambda (spec-phrase)
                       (:if (prelude:null spec-phrase)
                            (prelude:raise typespec 'prelude:typep "typespec format")
                            (prelude:typep value (mu:car (mu:cdr spec-phrase)))))
                     (prelude:%assoc typespec prelude:%typespec-map%))
                    (:if (prelude:consp typespec)
                         ((:lambda (spec spec-list)
                            ((:lambda (spec-phrase)
                               (:if (prelude:null spec-phrase)
                                    (prelude:raise typespec 'prelude:typep "typespec not found")
                                    (mu:apply (mu:cdr spec-phrase) `(,value ,spec-list))))
                             (prelude:%assoc spec prelude:%compound-typespec-map%)))
                          (mu:car typespec)
                          (mu:cdr typespec))
                         (prelude:raise typespec 'prelude:typep "typespec format")))))))
;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; vectors
;;;
(mu:intern :prelude "vectorp"
   (:lambda (vector)
     (:if (mu:eq :vector (mu:type-of vector))
          :t
          (mu:eq 'vector (prelude:type-of vector)))))

(mu:intern :prelude "bit-vector-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (mu:eq 'vector (prelude:type-of vector))
               (prelude:not (prelude:null (prelude:%vector-prop :bitvec vector)))
               ())
          ())))

(mu:intern :prelude "vector-displaced-p"
   (:lambda (vector)
     (:if (prelude:%prelude-type-p vector)
          (:if (mu:eq 'vector (prelude:type-of vector))
               (prelude:%vector-prop :disp vector)
               ())
          ())))

(mu:intern :prelude "make-vector"
   (:lambda (list)
     (:if (prelude:listp list)
          (mu:vector (prelude:%specialized-vector-type list) list)
          (prelude:raise list 'prelude:make-vector "not a list"))))

(mu:intern :prelude "slice"
   (:lambda (vector start length)
     (:if (prelude:typep vector :vector)
          (:if (prelude:fixnump start)
               (:if (prelude:%orf (prelude:minusp start) (mu:fx-lt (prelude:vector-length vector) start))
                    (prelude:raise start 'prelude:slice "illegal start")
                    (:if (prelude:fixnump length)
                         (:if (prelude:%orf (prelude:minusp length) (mu:fx-lt (prelude:vector-length vector) length))
                              (prelude:raise length 'prelude:slice "illegal length")
                              (prelude:%make-vector vector `(,start ,@length) ()))
                         (prelude:raise start 'prelude:slice "not a fixnum")))
               (prelude:raise length 'prelude:slice "not a fixnum"))
          (prelude:raise vector 'prelude:slice "not a base vector"))))

(mu:intern :prelude "vector-length"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               (mu:car (prelude:%vector-prop :bitvec vector))
               (:if (prelude:typep vector :vector)
                    (mu:sv-len vector)
                    (mu:cdr (prelude:%vector-prop :disp vector))))
          (prelude:raise vector 'prelude:vector-length "not a vector"))))

(mu:intern :prelude "vector-type"
   (:lambda (vector)
     (:if (prelude:vectorp vector)
          (:if (prelude:bit-vector-p vector)
               'bit
               (:if (prelude:typep vector :vector)
                    (mu:sv-type vector)
                    (mu:sv-type (prelude:%vector-prop :base vector))))
          (prelude:raise vector 'prelude:vector-type "not a vector"))))

(mu:intern :prelude "vector-ref"
   (:lambda (vector nth)
     (:if (prelude:vectorp vector)
          (:if (prelude:fixnump nth)
               (:if (prelude:%orf (prelude:minusp nth) (mu:fx-lt (prelude:vector-length vector) nth))
                    (prelude:raise length 'prelude:vector-ref "illegal index")
                    (:if (prelude:bit-vector-p vector)
                         ((:lambda (nth-byte offset)
                            ((:lambda (byte)
                               (:if (prelude:zerop (mu:logand byte (mu:ash 1 (mu:fx-sub 7 offset))))
                                    0
                                    1))
                               (prelude:vector-ref (prelude:%vector-prop :base vector) nth-byte)))
                          (mu:fx-div nth 8)
                          (mu:fx-sub nth (mu:fx-mul 8 (mu:fx-div nth 8))))
                         (:if (prelude:typep vector :vector)
                              (mu:sv-ref vector nth)
                              ((:lambda (base start)
                                 (mu:sv-ref base (mu:fx-add start nth)))
                               (prelude:%vector-prop :base vector)
                               (mu:car (prelude:%vector-prop :disp vector))))))
                    (prelude:raise nth 'prelude:vector-ref "not a fixnum"))
          (prelude:raise prop 'prelude:vector-ref "not a vector"))))

;;;
;;; write
;;;
(mu:intern :prelude "%vector-write"
   (:lambda (vector escape stream)
       (:if (prelude:typep vector :vector)
           (mu:write vector escape stream)
           ((:lambda (length)
               (prelude:format stream "#(" ())
               (mu:fix
                (:lambda (index)
                   (:if (mu:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref vector index) escape stream)
                            (:if (mu:fx-lt index (prelude:1- length))
                                 (prelude:format stream " " ())
                                 ())
                            (prelude:1+ index)))
                        index))
                0)
               (prelude:format stream ")" ()))
            (prelude:vector-length vector)))
      ()))

;;;
;;; implementations
;;;
(mu:intern :prelude "%make-vector"
   (:lambda (vector displace bitvec)
     (prelude:%make-type "vector"
       `(,(mu:cons :base   vector)
         ,(mu:cons :disp   displace)
         ,(mu:cons :bitvec bitvec)))))

(mu:intern :prelude "%specialized-vector-type"
   (:lambda (list)
     ((:lambda (type)
        (:if (prelude:findl (:lambda (stype) (mu:eq type stype)) '(:t :char :fixnum :float))
             (:if (mu:eq type :fixnum)
                  ((:lambda (is-byte)
                     (:if is-byte
                          :byte
                          :fixnum))
                   (prelude:foldl
                    (:lambda (elt acc)
                      (:if (prelude:%andf (prelude:not (prelude:minusp elt)) (mu:fx-lt elt 256))
                           acc
                           ()))
                      :t
                      list))
                  type)
             :t))
      (prelude:foldl
       (:lambda (elt acc)
          (:if (mu:eq :t acc)
               :t
               (:if (mu:eq acc (mu:type-of elt))
                    acc
                    ())))
      (mu:type-of (mu:car list))
      (mu:cdr list)))))

(mu:intern :prelude "%vector-prop"
   (:lambda (prop vector)
      (:if (prelude:findl (:lambda (vec-prop) (mu:eq prop vec-prop)) '(:base :disp :bitvec))
           (mu:cdr (prelude:%type-ref prop vector))
           (prelude:raise prop 'prelude:%vector-prop "not a vector property"))))

(mu:intern :prelude "%map-vector"
  (:lambda (fn vector)
    (mu:fix
     (:lambda (nth)
       (:if nth
            (:if (mu:eq nth (mu:sv-len vector))
                 ()
                 ((:lambda ()
                     (prelude:apply fn `(,(mu:sv-ref vector nth)))
                     (mu:fx-add 1 nth))))
            ()))
     0)))
