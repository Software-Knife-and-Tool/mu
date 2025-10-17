;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; symbol readers
;;;
(mu:intern core "%read-keywordp"
  (:lambda (name)
      (mu:eq #\: (mu:svref name 0))))

(mu:intern core "%read-symbol-ns"
  (:lambda (name)
    ((:lambda (colon-pos)
       (:if (core:null colon-pos)
            :unqual
            ((:lambda (ns-designator)
               (:if (core:%and
                     (mu:eq 1 (mu:vector-length ns-designator))
                     (mu:eq #\# (mu:svref ns-designator 0)))
                    ()        
                    (mu:find-namespace ns-designator)))
             (core:%vector-substr name 0 (mu:sub colon-pos 1)))))
     (core:%vector-position #\: name))))

(mu:intern core "%read-symbol-name"
   (:lambda (name)
     ((:lambda (colon)
        (:if (core:null colon)
             name
             (core:%vector-substr
              name
              (mu:add 1 colon)
              (mu:add colon (mu:sub (mu:vector-length name) (mu:add 1 colon))))))
      (core:%vector-position #\: name))))

(mu:intern core "%read-symbol"
   (:lambda (symbol)
     (:if (core:%read-keywordp symbol)
          (core:%make-keyword (core:%read-symbol-name symbol))
          ((:lambda (ns name)
             (:if (core:null ns)
                  (mu:make-symbol name)
                  (:if (mu:eq :unqual ns)
                       ((:lambda (lexical)
                          (:if (core:null lexical)
                               (core:%mktemp symbol)
                               lexical))
                        (mu:find core:%reader/ name))
                       ((:lambda (symbol)
                          (:if (core:null symbol)
                               (mu:intern ns name ())
                               symbol))
                        (mu:find ns name)))))
             (core:%read-symbol-ns symbol)
             (core:%read-symbol-name symbol)))))

;;;
;;;  symbol predicates
;;;
(mu:intern core "%uninternedp"
   (:lambda (sym)
      (core:null (mu:symbol-namespace sym))))

(mu:intern core "symbolp"
   (:lambda (t)
     (core:%or
      (core:%or (core:keywordp t) (mu:eq () t))
      (core:%or (mu:eq :symbol (mu:type-of t)) (mu:eq 'symbol (core:type-of t))))))

;;;
;;; utilities
;;;
(mu:intern core "%make-keyword"
   (:lambda (name)
     (mu:intern (mu:find-namespace "keyword") name ())))

(mu:intern core "%mktemp"
   (:lambda (symbol)
     (mu:write symbol () core:%quote%)
     (mu:read core:%quote% () ())))

(mu:intern core "%gensym-counter%"
   (mu:open :string :bidir "0" :t))

(mu:intern core "gensym"
   (:lambda ()
      ((:lambda (counter)
          (mu:write (mu:add counter 1) () core:%gensym-counter%)
          (mu:make-symbol (core:%format () "g~X" (mu:cons counter ()))))
       (mu:read core:%gensym-counter% () ()))))

(mu:intern core "gentemp"
   (:lambda ()
     (core:%mktemp (mu:symbol-name (core:gensym)))))
