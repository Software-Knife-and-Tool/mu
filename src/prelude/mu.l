;;; delete me

(mu:intern mu:%null-ns% "list" (lambda (&rest list) list))

(mu:intern mu:%null-ns% "list*"
   (lambda (&rest args)
     (mu:car
      (mu:fix
       (:lambda (loop)
         ((:lambda (list args)
            (:if args
                 (:if (mu:cdr args)
                      (mu:cons (mu:append list (mu:cons (mu:car args) ())) (mu:cdr args))
                      (mu:cons (mu:append list (mu:car args)) ()))
                 loop))
          (mu:car loop)
          (mu:cdr loop)))
       (mu:cons () args)))))

(mu:intern mu:%null-ns% "list+" ;;; delete me
   (lambda (&rest args)
     (if args
         (if (mu:cdr args)
             (mu:cons (mu:car args) (list+ (mu:cdr args)))
             (mu:car args)))))

(%defmacro progn (&rest body) (list (list* 'lambda () body)))

#|
(%defmacro when (test &rest body)
  (list 'if test (list* 'progn body)))

(%defmacro unless (test &rest body)
  (list 'if test () (list* 'progn body)))
|#
